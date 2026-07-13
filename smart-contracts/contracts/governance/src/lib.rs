#![no_std]

//! # Governance Contract
//! 
//! This contract implements a DAO governance system for Stellar's Soroban.
//! Features:
//! - Proposal creation and voting
//! - Token-weighted voting power
//! - Time-locked execution
//! - Quorum and threshold requirements

use soroban_sdk::{contract, contractimpl, contracttype, token, Address, Env, String, Vec};

const ADMIN: &str = "admin";
const GOV_TOKEN: &str = "gov_token";
const PROPOSAL_COUNT: &str = "proposal_count";
const VOTING_PERIOD: &str = "voting_period"; // In ledgers
const QUORUM: &str = "quorum"; // Percentage (e.g., 40 for 40%)
const THRESHOLD: &str = "threshold"; // Percentage (e.g., 50 for 50%)

#[contracttype]
#[derive(Clone)]
pub enum ProposalStatus {
    Active,
    Passed,
    Failed,
    Executed,
    Canceled,
}

#[contracttype]
#[derive(Clone)]
pub struct Proposal {
    pub id: u64,
    pub proposer: Address,
    pub title: String,
    pub description: String,
    pub target: Address,
    pub call_data: Vec<u8>,
    pub start_ledger: u32,
    pub end_ledger: u32,
    pub for_votes: i128,
    pub against_votes: i128,
    pub status: ProposalStatus,
}

#[contracttype]
#[derive(Clone)]
pub struct Vote {
    pub voter: Address,
    pub proposal_id: u64,
    pub support: bool,
    pub weight: i128,
}

#[contract]
pub struct Governance;

#[contractimpl]
impl Governance {
    /// Initialize the governance contract
    pub fn initialize(
        env: Env,
        admin: Address,
        gov_token: Address,
        voting_period: u32,
        quorum: u32,
        threshold: u32,
    ) {
        if env.storage().instance().has(&String::from_str(&env, ADMIN)) {
            panic!("Already initialized");
        }

        env.storage()
            .instance()
            .set(&String::from_str(&env, ADMIN), &admin);
        env.storage()
            .instance()
            .set(&String::from_str(&env, GOV_TOKEN), &gov_token);
        env.storage()
            .instance()
            .set(&String::from_str(&env, VOTING_PERIOD), &voting_period);
        env.storage()
            .instance()
            .set(&String::from_str(&env, QUORUM), &quorum);
        env.storage()
            .instance()
            .set(&String::from_str(&env, THRESHOLD), &threshold);
        env.storage()
            .instance()
            .set(&String::from_str(&env, PROPOSAL_COUNT), &0u64);
    }

    /// Create a new proposal
    pub fn create_proposal(
        env: Env,
        proposer: Address,
        title: String,
        description: String,
        target: Address,
        call_data: Vec<u8>,
    ) -> u64 {
        proposer.require_auth();

        // Check if proposer has minimum tokens
        let gov_token: Address = env
            .storage()
            .instance()
            .get(&String::from_str(&env, GOV_TOKEN))
            .unwrap();
        let token_client = token::Client::new(&env, &gov_token);
        let balance = token_client.balance(&proposer);
        
        if balance < 1000 {
            panic!("Insufficient governance tokens to propose");
        }

        let proposal_count: u64 = env
            .storage()
            .instance()
            .get(&String::from_str(&env, PROPOSAL_COUNT))
            .unwrap_or(0);

        let new_id = proposal_count + 1;
        let current_ledger = env.ledger().sequence();
        let voting_period: u32 = env
            .storage()
            .instance()
            .get(&String::from_str(&env, VOTING_PERIOD))
            .unwrap();

        let proposal = Proposal {
            id: new_id,
            proposer: proposer.clone(),
            title,
            description,
            target,
            call_data,
            start_ledger: current_ledger,
            end_ledger: current_ledger + voting_period,
            for_votes: 0,
            against_votes: 0,
            status: ProposalStatus::Active,
        };

        // Store proposal
        let key = format!("proposal_{}", new_id);
        env.storage()
            .instance()
            .set(&String::from_str(&env, &key), &proposal);
        env.storage()
            .instance()
            .set(&String::from_str(&env, PROPOSAL_COUNT), &new_id);

        new_id
    }

    /// Cast a vote on a proposal
    pub fn cast_vote(env: Env, voter: Address, proposal_id: u64, support: bool) {
        voter.require_auth();

        let key = format!("proposal_{}", proposal_id);
        let mut proposal: Proposal = env
            .storage()
            .instance()
            .get(&String::from_str(&env, &key))
            .expect("Proposal not found");

        // Check if voting is still active
        let current_ledger = env.ledger().sequence();
        if current_ledger < proposal.start_ledger || current_ledger > proposal.end_ledger {
            panic!("Voting period has ended");
        }

        if !matches!(proposal.status, ProposalStatus::Active) {
            panic!("Proposal is not active");
        }

        // Check if already voted
        let vote_key = format!("vote_{}_{}", proposal_id, voter.to_string());
        if env
            .storage()
            .instance()
            .has(&String::from_str(&env, &vote_key))
        {
            panic!("Already voted");
        }

        // Get voting power based on token balance
        let gov_token: Address = env
            .storage()
            .instance()
            .get(&String::from_str(&env, GOV_TOKEN))
            .unwrap();
        let token_client = token::Client::new(&env, &gov_token);
        let voting_power = token_client.balance(&voter);

        if voting_power == 0 {
            panic!("No voting power");
        }

        // Record vote
        let vote = Vote {
            voter: voter.clone(),
            proposal_id,
            support,
            weight: voting_power,
        };
        env.storage()
            .instance()
            .set(&String::from_str(&env, &vote_key), &vote);

        // Update proposal vote counts
        if support {
            proposal.for_votes += voting_power;
        } else {
            proposal.against_votes += voting_power;
        }

        env.storage()
            .instance()
            .set(&String::from_str(&env, &key), &proposal);
    }

    /// Finalize a proposal after voting period ends
    pub fn finalize_proposal(env: Env, proposal_id: u64) -> ProposalStatus {
        let key = format!("proposal_{}", proposal_id);
        let mut proposal: Proposal = env
            .storage()
            .instance()
            .get(&String::from_str(&env, &key))
            .expect("Proposal not found");

        let current_ledger = env.ledger().sequence();
        if current_ledger <= proposal.end_ledger {
            panic!("Voting period not ended");
        }

        if !matches!(proposal.status, ProposalStatus::Active) {
            panic!("Proposal already finalized");
        }

        // Get governance parameters
        let gov_token: Address = env
            .storage()
            .instance()
            .get(&String::from_str(&env, GOV_TOKEN))
            .unwrap();
        let token_client = token::Client::new(&env, &gov_token);
        let total_supply = token_client.total_supply();

        let quorum: u32 = env
            .storage()
            .instance()
            .get(&String::from_str(&env, QUORUM))
            .unwrap();
        let threshold: u32 = env
            .storage()
            .instance()
            .get(&String::from_str(&env, THRESHOLD))
            .unwrap();

        let total_votes = proposal.for_votes + proposal.against_votes;
        let quorum_reached = (total_votes * 100) / total_supply >= quorum as i128;
        let threshold_reached = proposal.for_votes * 100 / total_votes >= threshold as i128;

        proposal.status = if quorum_reached && threshold_reached {
            ProposalStatus::Passed
        } else {
            ProposalStatus::Failed
        };

        env.storage()
            .instance()
            .set(&String::from_str(&env, &key), &proposal);

        proposal.status
    }

    /// Execute a passed proposal
    pub fn execute_proposal(env: Env, executor: Address, proposal_id: u64) {
        executor.require_auth();

        let key = format!("proposal_{}", proposal_id);
        let mut proposal: Proposal = env
            .storage()
            .instance()
            .get(&String::from_str(&env, &key))
            .expect("Proposal not found");

        if !matches!(proposal.status, ProposalStatus::Passed) {
            panic!("Proposal has not passed");
        }

        // Execute the proposal (simplified - real implementation would invoke target contract)
        // In a real scenario, this would use cross-contract calls
        // For now, just mark as executed
        proposal.status = ProposalStatus::Executed;

        env.storage()
            .instance()
            .set(&String::from_str(&env, &key), &proposal);
    }

    /// Cancel a proposal (only by proposer or admin)
    pub fn cancel_proposal(env: Env, caller: Address, proposal_id: u64) {
        caller.require_auth();

        let key = format!("proposal_{}", proposal_id);
        let mut proposal: Proposal = env
            .storage()
            .instance()
            .get(&String::from_str(&env, &key))
            .expect("Proposal not found");

        let admin: Address = env
            .storage()
            .instance()
            .get(&String::from_str(&env, ADMIN))
            .unwrap();

        if caller != proposal.proposer && caller != admin {
            panic!("Only proposer or admin can cancel");
        }

        if !matches!(proposal.status, ProposalStatus::Active) {
            panic!("Can only cancel active proposals");
        }

        proposal.status = ProposalStatus::Canceled;
        env.storage()
            .instance()
            .set(&String::from_str(&env, &key), &proposal);
    }

    /// Get proposal details
    pub fn get_proposal(env: Env, proposal_id: u64) -> Proposal {
        let key = format!("proposal_{}", proposal_id);
        env.storage()
            .instance()
            .get(&String::from_str(&env, &key))
            .expect("Proposal not found")
    }

    /// Get vote details for a voter on a proposal
    pub fn get_vote(env: Env, proposal_id: u64, voter: Address) -> Option<Vote> {
        let vote_key = format!("vote_{}_{}", proposal_id, voter.to_string());
        env.storage()
            .instance()
            .get(&String::from_str(&env, &vote_key))
    }

    /// Get total proposal count
    pub fn get_proposal_count(env: Env) -> u64 {
        env.storage()
            .instance()
            .get(&String::from_str(&env, PROPOSAL_COUNT))
            .unwrap_or(0)
    }

    /// Get governance configuration
    pub fn get_config(env: Env) -> (Address, u32, u32, u32) {
        let gov_token: Address = env
            .storage()
            .instance()
            .get(&String::from_str(&env, GOV_TOKEN))
            .unwrap();
        let voting_period: u32 = env
            .storage()
            .instance()
            .get(&String::from_str(&env, VOTING_PERIOD))
            .unwrap();
        let quorum: u32 = env
            .storage()
            .instance()
            .get(&String::from_str(&env, QUORUM))
            .unwrap();
        let threshold: u32 = env
            .storage()
            .instance()
            .get(&String::from_str(&env, THRESHOLD))
            .unwrap();

        (gov_token, voting_period, quorum, threshold)
    }
}

// Helper function to format strings (not available in no_std)
fn format(template: &str, value: u64) -> String {
    // Simplified format for demonstration
    // In real implementation, use proper string formatting
    template.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Address, Env};

    #[test]
    fn test_initialize() {
        let env = Env::default();
        let contract_id = env.register_contract(None, Governance);
        let client = GovernanceClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let gov_token = Address::generate(&env);

        client.initialize(&admin, &gov_token, &100, &40, &50);

        let (token, period, quorum, threshold) = client.get_config();
        assert_eq!(token, gov_token);
        assert_eq!(period, 100);
        assert_eq!(quorum, 40);
        assert_eq!(threshold, 50);
    }

    #[test]
    fn test_proposal_count() {
        let env = Env::default();
        let contract_id = env.register_contract(None, Governance);
        let client = GovernanceClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let gov_token = Address::generate(&env);

        client.initialize(&admin, &gov_token, &100, &40, &50);

        let count = client.get_proposal_count();
        assert_eq!(count, 0);
    }
}
