#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, Address, Env, String, Symbol, Vec, Map,
};

#[derive(Clone)]
#[contracttype]
pub struct NFTMetadata {
    pub name: String,
    pub description: String,
    pub uri: String,
}

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Admin,
    TokenOwner(u64),      // token_id -> owner
    TokenMetadata(u64),   // token_id -> metadata
    OwnerTokens(Address), // owner -> Vec<token_id>
    TotalSupply,
    NextTokenId,
    CollectionName,
    CollectionSymbol,
}

#[contract]
pub struct NFTContract;

#[contractimpl]
impl NFTContract {
    /// Initialize the NFT collection
    pub fn initialize(env: Env, admin: Address, name: String, symbol: String) {
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("Already initialized");
        }

        admin.require_auth();

        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::CollectionName, &name);
        env.storage().instance().set(&DataKey::CollectionSymbol, &symbol);
        env.storage().instance().set(&DataKey::TotalSupply, &0u64);
        env.storage().instance().set(&DataKey::NextTokenId, &1u64);
    }

    /// Mint a new NFT
    pub fn mint(
        env: Env,
        to: Address,
        name: String,
        description: String,
        uri: String,
    ) -> u64 {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        admin.require_auth();

        let token_id: u64 = env
            .storage()
            .instance()
            .get(&DataKey::NextTokenId)
            .unwrap_or(1);

        // Create metadata
        let metadata = NFTMetadata {
            name,
            description,
            uri,
        };

        // Store token data
        env.storage()
            .persistent()
            .set(&DataKey::TokenOwner(token_id), &to);
        env.storage()
            .persistent()
            .set(&DataKey::TokenMetadata(token_id), &metadata);

        // Update owner's token list
        let mut owner_tokens: Vec<u64> = env
            .storage()
            .persistent()
            .get(&DataKey::OwnerTokens(to.clone()))
            .unwrap_or(Vec::new(&env));
        owner_tokens.push_back(token_id);
        env.storage()
            .persistent()
            .set(&DataKey::OwnerTokens(to.clone()), &owner_tokens);

        // Update counters
        let total_supply: u64 = env.storage().instance().get(&DataKey::TotalSupply).unwrap();
        env.storage()
            .instance()
            .set(&DataKey::TotalSupply, &(total_supply + 1));
        env.storage()
            .instance()
            .set(&DataKey::NextTokenId, &(token_id + 1));

        // Emit event
        env.events().publish(
            (Symbol::new(&env, "mint"), to),
            token_id,
        );

        token_id
    }

    /// Transfer NFT
    pub fn transfer(env: Env, from: Address, to: Address, token_id: u64) {
        from.require_auth();

        // Verify ownership
        let owner: Address = env
            .storage()
            .persistent()
            .get(&DataKey::TokenOwner(token_id))
            .expect("Token does not exist");

        if owner != from {
            panic!("Not token owner");
        }

        // Remove token from sender's list
        let mut from_tokens: Vec<u64> = env
            .storage()
            .persistent()
            .get(&DataKey::OwnerTokens(from.clone()))
            .unwrap();

        // Find and remove token_id
        let mut new_from_tokens = Vec::new(&env);
        for i in 0..from_tokens.len() {
            let tid = from_tokens.get(i).unwrap();
            if tid != token_id {
                new_from_tokens.push_back(tid);
            }
        }
        env.storage()
            .persistent()
            .set(&DataKey::OwnerTokens(from.clone()), &new_from_tokens);

        // Add token to recipient's list
        let mut to_tokens: Vec<u64> = env
            .storage()
            .persistent()
            .get(&DataKey::OwnerTokens(to.clone()))
            .unwrap_or(Vec::new(&env));
        to_tokens.push_back(token_id);
        env.storage()
            .persistent()
            .set(&DataKey::OwnerTokens(to.clone()), &to_tokens);

        // Update owner
        env.storage()
            .persistent()
            .set(&DataKey::TokenOwner(token_id), &to);

        // Emit event
        env.events().publish(
            (Symbol::new(&env, "transfer"), from, to),
            token_id,
        );
    }

    /// Burn NFT
    pub fn burn(env: Env, from: Address, token_id: u64) {
        from.require_auth();

        let owner: Address = env
            .storage()
            .persistent()
            .get(&DataKey::TokenOwner(token_id))
            .expect("Token does not exist");

        if owner != from {
            panic!("Not token owner");
        }

        // Remove token from owner's list
        let mut from_tokens: Vec<u64> = env
            .storage()
            .persistent()
            .get(&DataKey::OwnerTokens(from.clone()))
            .unwrap();

        let mut new_from_tokens = Vec::new(&env);
        for i in 0..from_tokens.len() {
            let tid = from_tokens.get(i).unwrap();
            if tid != token_id {
                new_from_tokens.push_back(tid);
            }
        }
        env.storage()
            .persistent()
            .set(&DataKey::OwnerTokens(from.clone()), &new_from_tokens);

        // Remove token data
        env.storage().persistent().remove(&DataKey::TokenOwner(token_id));
        env.storage().persistent().remove(&DataKey::TokenMetadata(token_id));

        // Update total supply
        let total_supply: u64 = env.storage().instance().get(&DataKey::TotalSupply).unwrap();
        env.storage()
            .instance()
            .set(&DataKey::TotalSupply, &(total_supply - 1));

        // Emit event
        env.events().publish(
            (Symbol::new(&env, "burn"), from),
            token_id,
        );
    }

    /// Get owner of token
    pub fn owner_of(env: Env, token_id: u64) -> Address {
        env.storage()
            .persistent()
            .get(&DataKey::TokenOwner(token_id))
            .expect("Token does not exist")
    }

    /// Get token metadata
    pub fn token_metadata(env: Env, token_id: u64) -> NFTMetadata {
        env.storage()
            .persistent()
            .get(&DataKey::TokenMetadata(token_id))
            .expect("Token does not exist")
    }

    /// Get tokens owned by address
    pub fn tokens_of(env: Env, owner: Address) -> Vec<u64> {
        env.storage()
            .persistent()
            .get(&DataKey::OwnerTokens(owner))
            .unwrap_or(Vec::new(&env))
    }

    /// Get total supply
    pub fn total_supply(env: Env) -> u64 {
        env.storage()
            .instance()
            .get(&DataKey::TotalSupply)
            .unwrap_or(0)
    }

    /// Get collection name
    pub fn name(env: Env) -> String {
        env.storage()
            .instance()
            .get(&DataKey::CollectionName)
            .unwrap()
    }

    /// Get collection symbol
    pub fn symbol(env: Env) -> String {
        env.storage()
            .instance()
            .get(&DataKey::CollectionSymbol)
            .unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Env};

    #[test]
    fn test_mint_and_transfer() {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register_contract(None, NFTContract);
        let client = NFTContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let user1 = Address::generate(&env);
        let user2 = Address::generate(&env);

        // Initialize
        client.initialize(
            &admin,
            &String::from_str(&env, "Test NFTs"),
            &String::from_str(&env, "TNFT"),
        );

        // Mint NFT
        let token_id = client.mint(
            &user1,
            &String::from_str(&env, "NFT #1"),
            &String::from_str(&env, "First NFT"),
            &String::from_str(&env, "ipfs://..."),
        );

        assert_eq!(token_id, 1);
        assert_eq!(client.owner_of(&token_id), user1);
        assert_eq!(client.total_supply(), 1);

        // Transfer
        client.transfer(&user1, &user2, &token_id);
        assert_eq!(client.owner_of(&token_id), user2);
    }

    #[test]
    fn test_burn() {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register_contract(None, NFTContract);
        let client = NFTContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let user = Address::generate(&env);

        client.initialize(
            &admin,
            &String::from_str(&env, "Test"),
            &String::from_str(&env, "TST"),
        );

        let token_id = client.mint(
            &user,
            &String::from_str(&env, "NFT"),
            &String::from_str(&env, "Desc"),
            &String::from_str(&env, "uri"),
        );

        client.burn(&user, &token_id);
        assert_eq!(client.total_supply(), 0);
    }
}
