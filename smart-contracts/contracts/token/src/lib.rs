#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, Address, Env, String, Symbol, Vec,
};
use soroban_token_sdk::{metadata::TokenMetadata, TokenUtils};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Admin,
    Balance(Address),
    Allowance(Address, Address),
    TotalSupply,
    Metadata(Symbol),
}

#[contract]
pub struct TokenContract;

#[contractimpl]
impl TokenContract {
    /// Initialize the token with metadata
    pub fn initialize(
        env: Env,
        admin: Address,
        name: String,
        symbol: String,
        decimals: u32,
    ) {
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("Already initialized");
        }

        admin.require_auth();

        // Store admin
        env.storage().instance().set(&DataKey::Admin, &admin);

        // Store metadata
        env.storage().instance().set(&DataKey::Metadata(Symbol::new(&env, "name")), &name);
        env.storage().instance().set(&DataKey::Metadata(Symbol::new(&env, "symbol")), &symbol);
        env.storage().instance().set(&DataKey::Metadata(Symbol::new(&env, "decimals")), &decimals);

        // Initialize total supply
        env.storage().instance().set(&DataKey::TotalSupply, &0i128);
    }

    /// Mint new tokens
    pub fn mint(env: Env, to: Address, amount: i128) {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        admin.require_auth();

        if amount <= 0 {
            panic!("Amount must be positive");
        }

        // Get current balance
        let balance = Self::balance_of(env.clone(), to.clone());
        let new_balance = balance + amount;

        // Update balance
        env.storage()
            .persistent()
            .set(&DataKey::Balance(to.clone()), &new_balance);

        // Update total supply
        let total_supply: i128 = env.storage().instance().get(&DataKey::TotalSupply).unwrap_or(0);
        env.storage()
            .instance()
            .set(&DataKey::TotalSupply, &(total_supply + amount));

        // Emit event
        env.events().publish(
            (Symbol::new(&env, "mint"), to),
            amount,
        );
    }

    /// Burn tokens
    pub fn burn(env: Env, from: Address, amount: i128) {
        from.require_auth();

        if amount <= 0 {
            panic!("Amount must be positive");
        }

        let balance = Self::balance_of(env.clone(), from.clone());
        if balance < amount {
            panic!("Insufficient balance");
        }

        let new_balance = balance - amount;
        env.storage()
            .persistent()
            .set(&DataKey::Balance(from.clone()), &new_balance);

        // Update total supply
        let total_supply: i128 = env.storage().instance().get(&DataKey::TotalSupply).unwrap();
        env.storage()
            .instance()
            .set(&DataKey::TotalSupply, &(total_supply - amount));

        // Emit event
        env.events().publish(
            (Symbol::new(&env, "burn"), from),
            amount,
        );
    }

    /// Transfer tokens
    pub fn transfer(env: Env, from: Address, to: Address, amount: i128) {
        from.require_auth();

        if amount <= 0 {
            panic!("Amount must be positive");
        }

        let from_balance = Self::balance_of(env.clone(), from.clone());
        if from_balance < amount {
            panic!("Insufficient balance");
        }

        let to_balance = Self::balance_of(env.clone(), to.clone());

        env.storage()
            .persistent()
            .set(&DataKey::Balance(from.clone()), &(from_balance - amount));
        env.storage()
            .persistent()
            .set(&DataKey::Balance(to.clone()), &(to_balance + amount));

        // Emit event
        env.events().publish(
            (Symbol::new(&env, "transfer"), from, to),
            amount,
        );
    }

    /// Get balance of an address
    pub fn balance_of(env: Env, account: Address) -> i128 {
        env.storage()
            .persistent()
            .get(&DataKey::Balance(account))
            .unwrap_or(0)
    }

    /// Get total supply
    pub fn total_supply(env: Env) -> i128 {
        env.storage()
            .instance()
            .get(&DataKey::TotalSupply)
            .unwrap_or(0)
    }

    /// Get token name
    pub fn name(env: Env) -> String {
        env.storage()
            .instance()
            .get(&DataKey::Metadata(Symbol::new(&env, "name")))
            .unwrap()
    }

    /// Get token symbol
    pub fn symbol(env: Env) -> String {
        env.storage()
            .instance()
            .get(&DataKey::Metadata(Symbol::new(&env, "symbol")))
            .unwrap()
    }

    /// Get token decimals
    pub fn decimals(env: Env) -> u32 {
        env.storage()
            .instance()
            .get(&DataKey::Metadata(Symbol::new(&env, "decimals")))
            .unwrap()
    }

    /// Approve spender to spend tokens
    pub fn approve(env: Env, owner: Address, spender: Address, amount: i128) {
        owner.require_auth();

        env.storage()
            .persistent()
            .set(&DataKey::Allowance(owner.clone(), spender.clone()), &amount);

        env.events().publish(
            (Symbol::new(&env, "approve"), owner, spender),
            amount,
        );
    }

    /// Get allowance
    pub fn allowance(env: Env, owner: Address, spender: Address) -> i128 {
        env.storage()
            .persistent()
            .get(&DataKey::Allowance(owner, spender))
            .unwrap_or(0)
    }

    /// Transfer from approved allowance
    pub fn transfer_from(env: Env, spender: Address, from: Address, to: Address, amount: i128) {
        spender.require_auth();

        let allowance = Self::allowance(env.clone(), from.clone(), spender.clone());
        if allowance < amount {
            panic!("Insufficient allowance");
        }

        let from_balance = Self::balance_of(env.clone(), from.clone());
        if from_balance < amount {
            panic!("Insufficient balance");
        }

        let to_balance = Self::balance_of(env.clone(), to.clone());

        // Update balances
        env.storage()
            .persistent()
            .set(&DataKey::Balance(from.clone()), &(from_balance - amount));
        env.storage()
            .persistent()
            .set(&DataKey::Balance(to.clone()), &(to_balance + amount));

        // Update allowance
        env.storage()
            .persistent()
            .set(&DataKey::Allowance(from.clone(), spender.clone()), &(allowance - amount));

        env.events().publish(
            (Symbol::new(&env, "transfer_from"), spender, from, to),
            amount,
        );
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Env};

    #[test]
    fn test_initialize() {
        let env = Env::default();
        let contract_id = env.register_contract(None, TokenContract);
        let client = TokenContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let name = String::from_str(&env, "Test Token");
        let symbol = String::from_str(&env, "TEST");

        client.initialize(&admin, &name, &symbol, &18);

        assert_eq!(client.name(), name);
        assert_eq!(client.symbol(), symbol);
        assert_eq!(client.decimals(), 18);
        assert_eq!(client.total_supply(), 0);
    }

    #[test]
    fn test_mint_and_transfer() {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register_contract(None, TokenContract);
        let client = TokenContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let user1 = Address::generate(&env);
        let user2 = Address::generate(&env);

        // Initialize
        client.initialize(
            &admin,
            &String::from_str(&env, "Test"),
            &String::from_str(&env, "TST"),
            &18,
        );

        // Mint tokens
        client.mint(&user1, &1000);
        assert_eq!(client.balance_of(&user1), 1000);
        assert_eq!(client.total_supply(), 1000);

        // Transfer
        client.transfer(&user1, &user2, &500);
        assert_eq!(client.balance_of(&user1), 500);
        assert_eq!(client.balance_of(&user2), 500);
    }

    #[test]
    fn test_approve_and_transfer_from() {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register_contract(None, TokenContract);
        let client = TokenContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let owner = Address::generate(&env);
        let spender = Address::generate(&env);
        let recipient = Address::generate(&env);

        client.initialize(
            &admin,
            &String::from_str(&env, "Test"),
            &String::from_str(&env, "TST"),
            &18,
        );

        client.mint(&owner, &1000);
        client.approve(&owner, &spender, &500);

        assert_eq!(client.allowance(&owner, &spender), 500);

        client.transfer_from(&spender, &owner, &recipient, &300);

        assert_eq!(client.balance_of(&owner), 700);
        assert_eq!(client.balance_of(&recipient), 300);
        assert_eq!(client.allowance(&owner, &spender), 200);
    }
}
