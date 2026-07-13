#![no_std]

//! # DeFi Liquidity Pool Contract
//! 
//! This contract implements a liquidity pool for decentralized trading on Stellar's Soroban.
//! Features:
//! - Add/remove liquidity with LP tokens
//! - Token swaps with automated market making
//! - Fee collection for liquidity providers
//! - Slippage protection

use soroban_sdk::{contract, contractimpl, contracttype, token, Address, Env, String};

const POOL_TOKEN: &str = "pool_token";
const TOKEN_A: &str = "token_a";
const TOKEN_B: &str = "token_b";
const RESERVE_A: &str = "reserve_a";
const RESERVE_B: &str = "reserve_b";
const TOTAL_SHARES: &str = "total_shares";
const FEE_RATE: &str = "fee_rate"; // Basis points (100 = 1%)

#[contracttype]
#[derive(Clone)]
pub struct PoolInfo {
    pub token_a: Address,
    pub token_b: Address,
    pub reserve_a: i128,
    pub reserve_b: i128,
    pub total_shares: i128,
    pub fee_rate: u32,
}

#[contract]
pub struct DeFiPool;

#[contractimpl]
impl DeFiPool {
    /// Initialize the liquidity pool with two tokens
    pub fn initialize(
        env: Env,
        admin: Address,
        token_a: Address,
        token_b: Address,
        pool_token: Address,
        fee_rate: u32,
    ) {
        if env.storage().instance().has(&String::from_str(&env, TOKEN_A)) {
            panic!("Pool already initialized");
        }

        // Store pool configuration
        env.storage()
            .instance()
            .set(&String::from_str(&env, TOKEN_A), &token_a);
        env.storage()
            .instance()
            .set(&String::from_str(&env, TOKEN_B), &token_b);
        env.storage()
            .instance()
            .set(&String::from_str(&env, POOL_TOKEN), &pool_token);
        env.storage()
            .instance()
            .set(&String::from_str(&env, FEE_RATE), &fee_rate);

        // Initialize reserves
        env.storage()
            .instance()
            .set(&String::from_str(&env, RESERVE_A), &0i128);
        env.storage()
            .instance()
            .set(&String::from_str(&env, RESERVE_B), &0i128);
        env.storage()
            .instance()
            .set(&String::from_str(&env, TOTAL_SHARES), &0i128);
    }

    /// Add liquidity to the pool and receive LP tokens
    pub fn add_liquidity(
        env: Env,
        sender: Address,
        amount_a: i128,
        amount_b: i128,
        min_shares: i128,
    ) -> i128 {
        sender.require_auth();

        let token_a: Address = env
            .storage()
            .instance()
            .get(&String::from_str(&env, TOKEN_A))
            .unwrap();
        let token_b: Address = env
            .storage()
            .instance()
            .get(&String::from_str(&env, TOKEN_B))
            .unwrap();
        let pool_token: Address = env
            .storage()
            .instance()
            .get(&String::from_str(&env, POOL_TOKEN))
            .unwrap();

        let reserve_a: i128 = env
            .storage()
            .instance()
            .get(&String::from_str(&env, RESERVE_A))
            .unwrap_or(0);
        let reserve_b: i128 = env
            .storage()
            .instance()
            .get(&String::from_str(&env, RESERVE_B))
            .unwrap_or(0);
        let total_shares: i128 = env
            .storage()
            .instance()
            .get(&String::from_str(&env, TOTAL_SHARES))
            .unwrap_or(0);

        // Calculate shares to mint
        let shares = if total_shares == 0 {
            // First liquidity provision
            (amount_a * amount_b).sqrt()
        } else {
            // Proportional to existing pool
            let shares_a = (amount_a * total_shares) / reserve_a;
            let shares_b = (amount_b * total_shares) / reserve_b;
            shares_a.min(shares_b)
        };

        if shares < min_shares {
            panic!("Insufficient shares received");
        }

        // Transfer tokens from sender to contract
        let client_a = token::Client::new(&env, &token_a);
        let client_b = token::Client::new(&env, &token_b);
        client_a.transfer(&sender, &env.current_contract_address(), &amount_a);
        client_b.transfer(&sender, &env.current_contract_address(), &amount_b);

        // Mint LP tokens
        let pool_client = token::Client::new(&env, &pool_token);
        pool_client.mint(&sender, &shares);

        // Update reserves and total shares
        env.storage()
            .instance()
            .set(&String::from_str(&env, RESERVE_A), &(reserve_a + amount_a));
        env.storage()
            .instance()
            .set(&String::from_str(&env, RESERVE_B), &(reserve_b + amount_b));
        env.storage()
            .instance()
            .set(&String::from_str(&env, TOTAL_SHARES), &(total_shares + shares));

        shares
    }

    /// Remove liquidity from the pool by burning LP tokens
    pub fn remove_liquidity(
        env: Env,
        sender: Address,
        shares: i128,
        min_amount_a: i128,
        min_amount_b: i128,
    ) -> (i128, i128) {
        sender.require_auth();

        let token_a: Address = env
            .storage()
            .instance()
            .get(&String::from_str(&env, TOKEN_A))
            .unwrap();
        let token_b: Address = env
            .storage()
            .instance()
            .get(&String::from_str(&env, TOKEN_B))
            .unwrap();
        let pool_token: Address = env
            .storage()
            .instance()
            .get(&String::from_str(&env, POOL_TOKEN))
            .unwrap();

        let reserve_a: i128 = env
            .storage()
            .instance()
            .get(&String::from_str(&env, RESERVE_A))
            .unwrap();
        let reserve_b: i128 = env
            .storage()
            .instance()
            .get(&String::from_str(&env, RESERVE_B))
            .unwrap();
        let total_shares: i128 = env
            .storage()
            .instance()
            .get(&String::from_str(&env, TOTAL_SHARES))
            .unwrap();

        // Calculate amounts to return
        let amount_a = (shares * reserve_a) / total_shares;
        let amount_b = (shares * reserve_b) / total_shares;

        if amount_a < min_amount_a || amount_b < min_amount_b {
            panic!("Insufficient output amount");
        }

        // Burn LP tokens
        let pool_client = token::Client::new(&env, &pool_token);
        pool_client.burn(&sender, &shares);

        // Transfer tokens back to sender
        let client_a = token::Client::new(&env, &token_a);
        let client_b = token::Client::new(&env, &token_b);
        client_a.transfer(&env.current_contract_address(), &sender, &amount_a);
        client_b.transfer(&env.current_contract_address(), &sender, &amount_b);

        // Update reserves and total shares
        env.storage()
            .instance()
            .set(&String::from_str(&env, RESERVE_A), &(reserve_a - amount_a));
        env.storage()
            .instance()
            .set(&String::from_str(&env, RESERVE_B), &(reserve_b - amount_b));
        env.storage()
            .instance()
            .set(&String::from_str(&env, TOTAL_SHARES), &(total_shares - shares));

        (amount_a, amount_b)
    }

    /// Swap token A for token B
    pub fn swap_a_to_b(
        env: Env,
        sender: Address,
        amount_in: i128,
        min_amount_out: i128,
    ) -> i128 {
        sender.require_auth();

        let token_a: Address = env
            .storage()
            .instance()
            .get(&String::from_str(&env, TOKEN_A))
            .unwrap();
        let token_b: Address = env
            .storage()
            .instance()
            .get(&String::from_str(&env, TOKEN_B))
            .unwrap();
        let fee_rate: u32 = env
            .storage()
            .instance()
            .get(&String::from_str(&env, FEE_RATE))
            .unwrap();

        let reserve_a: i128 = env
            .storage()
            .instance()
            .get(&String::from_str(&env, RESERVE_A))
            .unwrap();
        let reserve_b: i128 = env
            .storage()
            .instance()
            .get(&String::from_str(&env, RESERVE_B))
            .unwrap();

        // Calculate output amount with fee
        let amount_in_with_fee = amount_in * (10000 - fee_rate as i128) / 10000;
        let amount_out = (amount_in_with_fee * reserve_b) / (reserve_a + amount_in_with_fee);

        if amount_out < min_amount_out {
            panic!("Insufficient output amount");
        }

        // Transfer tokens
        let client_a = token::Client::new(&env, &token_a);
        let client_b = token::Client::new(&env, &token_b);
        client_a.transfer(&sender, &env.current_contract_address(), &amount_in);
        client_b.transfer(&env.current_contract_address(), &sender, &amount_out);

        // Update reserves
        env.storage()
            .instance()
            .set(&String::from_str(&env, RESERVE_A), &(reserve_a + amount_in));
        env.storage()
            .instance()
            .set(&String::from_str(&env, RESERVE_B), &(reserve_b - amount_out));

        amount_out
    }

    /// Swap token B for token A
    pub fn swap_b_to_a(
        env: Env,
        sender: Address,
        amount_in: i128,
        min_amount_out: i128,
    ) -> i128 {
        sender.require_auth();

        let token_a: Address = env
            .storage()
            .instance()
            .get(&String::from_str(&env, TOKEN_A))
            .unwrap();
        let token_b: Address = env
            .storage()
            .instance()
            .get(&String::from_str(&env, TOKEN_B))
            .unwrap();
        let fee_rate: u32 = env
            .storage()
            .instance()
            .get(&String::from_str(&env, FEE_RATE))
            .unwrap();

        let reserve_a: i128 = env
            .storage()
            .instance()
            .get(&String::from_str(&env, RESERVE_A))
            .unwrap();
        let reserve_b: i128 = env
            .storage()
            .instance()
            .get(&String::from_str(&env, RESERVE_B))
            .unwrap();

        // Calculate output amount with fee
        let amount_in_with_fee = amount_in * (10000 - fee_rate as i128) / 10000;
        let amount_out = (amount_in_with_fee * reserve_a) / (reserve_b + amount_in_with_fee);

        if amount_out < min_amount_out {
            panic!("Insufficient output amount");
        }

        // Transfer tokens
        let client_a = token::Client::new(&env, &token_a);
        let client_b = token::Client::new(&env, &token_b);
        client_b.transfer(&sender, &env.current_contract_address(), &amount_in);
        client_a.transfer(&env.current_contract_address(), &sender, &amount_out);

        // Update reserves
        env.storage()
            .instance()
            .set(&String::from_str(&env, RESERVE_A), &(reserve_a - amount_out));
        env.storage()
            .instance()
            .set(&String::from_str(&env, RESERVE_B), &(reserve_b + amount_in));

        amount_out
    }

    /// Get current pool information
    pub fn get_pool_info(env: Env) -> PoolInfo {
        PoolInfo {
            token_a: env
                .storage()
                .instance()
                .get(&String::from_str(&env, TOKEN_A))
                .unwrap(),
            token_b: env
                .storage()
                .instance()
                .get(&String::from_str(&env, TOKEN_B))
                .unwrap(),
            reserve_a: env
                .storage()
                .instance()
                .get(&String::from_str(&env, RESERVE_A))
                .unwrap_or(0),
            reserve_b: env
                .storage()
                .instance()
                .get(&String::from_str(&env, RESERVE_B))
                .unwrap_or(0),
            total_shares: env
                .storage()
                .instance()
                .get(&String::from_str(&env, TOTAL_SHARES))
                .unwrap_or(0),
            fee_rate: env
                .storage()
                .instance()
                .get(&String::from_str(&env, FEE_RATE))
                .unwrap_or(30),
        }
    }

    /// Get quote for swapping token A to B
    pub fn get_quote_a_to_b(env: Env, amount_in: i128) -> i128 {
        let reserve_a: i128 = env
            .storage()
            .instance()
            .get(&String::from_str(&env, RESERVE_A))
            .unwrap();
        let reserve_b: i128 = env
            .storage()
            .instance()
            .get(&String::from_str(&env, RESERVE_B))
            .unwrap();
        let fee_rate: u32 = env
            .storage()
            .instance()
            .get(&String::from_str(&env, FEE_RATE))
            .unwrap();

        let amount_in_with_fee = amount_in * (10000 - fee_rate as i128) / 10000;
        (amount_in_with_fee * reserve_b) / (reserve_a + amount_in_with_fee)
    }

    /// Get quote for swapping token B to A
    pub fn get_quote_b_to_a(env: Env, amount_in: i128) -> i128 {
        let reserve_a: i128 = env
            .storage()
            .instance()
            .get(&String::from_str(&env, RESERVE_A))
            .unwrap();
        let reserve_b: i128 = env
            .storage()
            .instance()
            .get(&String::from_str(&env, RESERVE_B))
            .unwrap();
        let fee_rate: u32 = env
            .storage()
            .instance()
            .get(&String::from_str(&env, FEE_RATE))
            .unwrap();

        let amount_in_with_fee = amount_in * (10000 - fee_rate as i128) / 10000;
        (amount_in_with_fee * reserve_a) / (reserve_b + amount_in_with_fee)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Address, Env};

    #[test]
    fn test_initialize() {
        let env = Env::default();
        let contract_id = env.register_contract(None, DeFiPool);
        let client = DeFiPoolClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let token_a = Address::generate(&env);
        let token_b = Address::generate(&env);
        let pool_token = Address::generate(&env);

        client.initialize(&admin, &token_a, &token_b, &pool_token, &30);

        let pool_info = client.get_pool_info();
        assert_eq!(pool_info.token_a, token_a);
        assert_eq!(pool_info.token_b, token_b);
        assert_eq!(pool_info.reserve_a, 0);
        assert_eq!(pool_info.reserve_b, 0);
        assert_eq!(pool_info.total_shares, 0);
        assert_eq!(pool_info.fee_rate, 30);
    }

    #[test]
    fn test_get_quote() {
        let env = Env::default();
        let contract_id = env.register_contract(None, DeFiPool);
        let client = DeFiPoolClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let token_a = Address::generate(&env);
        let token_b = Address::generate(&env);
        let pool_token = Address::generate(&env);

        client.initialize(&admin, &token_a, &token_b, &pool_token, &30);

        // Manually set reserves for testing
        env.storage()
            .instance()
            .set(&String::from_str(&env, RESERVE_A), &1000000i128);
        env.storage()
            .instance()
            .set(&String::from_str(&env, RESERVE_B), &2000000i128);

        let quote = client.get_quote_a_to_b(&10000);
        assert!(quote > 0);
        assert!(quote < 20000); // Should be less than 2:1 ratio due to fees
    }
}
