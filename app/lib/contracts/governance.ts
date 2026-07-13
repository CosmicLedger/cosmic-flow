/**
 * Governance Contract Client
 * Wrapper for interacting with the Soroban Governance contract
 */

import { SorobanContractClient, ContractClientConfig } from './client';

export enum ProposalStatus {
  Active = 'Active',
  Passed = 'Passed',
  Failed = 'Failed',
  Executed = 'Executed',
  Canceled = 'Canceled',
}

export interface Proposal {
  id: bigint;
  proposer: string;
  title: string;
  description: string;
  target: string;
  call_data: Uint8Array;
  start_ledger: number;
  end_ledger: number;
  for_votes: bigint;
  against_votes: bigint;
  status: ProposalStatus;
}

export interface Vote {
  voter: string;
  proposal_id: bigint;
  support: boolean;
  weight: bigint;
}

export interface GovernanceConfig {
  gov_token: string;
  voting_period: number;
  quorum: number;
  threshold: number;
}

export class GovernanceContract extends SorobanContractClient {
  constructor(config: ContractClientConfig) {
    super(config);
  }

  /**
   * Get governance configuration
   */
  async getConfig(): Promise<GovernanceConfig> {
    const [gov_token, voting_period, quorum, threshold] = await this.read('get_config');
    return { gov_token, voting_period, quorum, threshold };
  }

  /**
   * Get total proposal count
   */
  async getProposalCount(): Promise<bigint> {
    return await this.read('get_proposal_count');
  }

  /**
   * Get proposal details
   */
  async getProposal(proposalId: bigint): Promise<Proposal> {
    return await this.read('get_proposal', proposalId);
  }

  /**
   * Get vote details
   */
  async getVote(proposalId: bigint, voter: string): Promise<Vote | null> {
    return await this.read('get_vote', proposalId, voter);
  }

  /**
   * Create a new proposal
   */
  async createProposal(
    sourceKeypair: any,
    proposer: string,
    title: string,
    description: string,
    target: string,
    callData: Uint8Array
  ): Promise<bigint> {
    return await this.write(
      'create_proposal',
      sourceKeypair,
      proposer,
      title,
      description,
      target,
      Array.from(callData)
    );
  }

  /**
   * Cast a vote on a proposal
   */
  async castVote(
    sourceKeypair: any,
    voter: string,
    proposalId: bigint,
    support: boolean
  ): Promise<void> {
    await this.write('cast_vote', sourceKeypair, voter, proposalId, support);
  }

  /**
   * Finalize a proposal after voting period
   */
  async finalizeProposal(
    sourceKeypair: any,
    proposalId: bigint
  ): Promise<ProposalStatus> {
    return await this.write('finalize_proposal', sourceKeypair, proposalId);
  }

  /**
   * Execute a passed proposal
   */
  async executeProposal(
    sourceKeypair: any,
    executor: string,
    proposalId: bigint
  ): Promise<void> {
    await this.write('execute_proposal', sourceKeypair, executor, proposalId);
  }

  /**
   * Cancel a proposal
   */
  async cancelProposal(
    sourceKeypair: any,
    caller: string,
    proposalId: bigint
  ): Promise<void> {
    await this.write('cancel_proposal', sourceKeypair, caller, proposalId);
  }

  /**
   * Check if an account can vote
   */
  async canVote(proposalId: bigint, voter: string): Promise<boolean> {
    try {
      const proposal = await this.getProposal(proposalId);
      const existingVote = await this.getVote(proposalId, voter);
      
      // Check if already voted
      if (existingVote !== null) {
        return false;
      }

      // Check if voting period is active
      // Note: You would need to get current ledger from server
      return proposal.status === ProposalStatus.Active;
    } catch (error) {
      return false;
    }
  }

  /**
   * Calculate voting power for an address
   */
  async getVotingPower(voter: string, tokenContract: any): Promise<bigint> {
    return await tokenContract.balance(voter);
  }

  /**
   * Get proposal vote participation percentage
   */
  calculateParticipation(
    forVotes: bigint,
    againstVotes: bigint,
    totalSupply: bigint
  ): number {
    const totalVotes = forVotes + againstVotes;
    return Number((totalVotes * BigInt(10000)) / totalSupply) / 100;
  }

  /**
   * Get proposal approval percentage
   */
  calculateApproval(forVotes: bigint, againstVotes: bigint): number {
    const totalVotes = forVotes + againstVotes;
    if (totalVotes === BigInt(0)) return 0;
    return Number((forVotes * BigInt(10000)) / totalVotes) / 100;
  }

  /**
   * Check if proposal will pass
   */
  willProposalPass(
    proposal: Proposal,
    totalSupply: bigint,
    quorum: number,
    threshold: number
  ): { passes: boolean; reason?: string } {
    const participation = this.calculateParticipation(
      proposal.for_votes,
      proposal.against_votes,
      totalSupply
    );
    const approval = this.calculateApproval(
      proposal.for_votes,
      proposal.against_votes
    );

    if (participation < quorum) {
      return { passes: false, reason: `Quorum not met (${participation.toFixed(2)}% < ${quorum}%)` };
    }

    if (approval < threshold) {
      return { passes: false, reason: `Threshold not met (${approval.toFixed(2)}% < ${threshold}%)` };
    }

    return { passes: true };
  }
}

/**
 * Format proposal status for display
 */
export function formatProposalStatus(status: ProposalStatus): string {
  return status.toString();
}

/**
 * Get proposal status color
 */
export function getProposalStatusColor(status: ProposalStatus): string {
  switch (status) {
    case ProposalStatus.Active:
      return 'blue';
    case ProposalStatus.Passed:
      return 'green';
    case ProposalStatus.Failed:
      return 'red';
    case ProposalStatus.Executed:
      return 'purple';
    case ProposalStatus.Canceled:
      return 'gray';
    default:
      return 'gray';
  }
}
