#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, Address, Env, String, Vec,
};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    Admin,
    NextTokenId,
    Token(u64),
    OwnerTokens(Address),
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LoyaltyNFT {
    pub token_id: u64,
    pub owner: Address,
    pub tier: String,
    pub points: u32,
    pub redeemed: bool,
    pub metadata_uri: String,
}

#[contract]
pub struct NftLoyaltyContract;

#[contractimpl]
impl NftLoyaltyContract {
    // Initialize contract with admin address
    pub fn initialize(env: Env, admin: Address) {
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("already initialized");
        }

        admin.require_auth();

        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::NextTokenId, &1u64);

        env.events().publish((symbol_short!("init"),), admin);
    }

    // Admin mints a loyalty NFT to a user
    pub fn mint(
        env: Env,
        to: Address,
        tier: String,
        points: u32,
        metadata_uri: String,
    ) -> u64 {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        admin.require_auth();

        let token_id: u64 = env.storage().instance().get(&DataKey::NextTokenId).unwrap();

        let nft = LoyaltyNFT {
            token_id,
            owner: to.clone(),
            tier: tier.clone(),
            points,
            redeemed: false,
            metadata_uri,
        };

        env.storage().persistent().set(&DataKey::Token(token_id), &nft);

        let owner_key = DataKey::OwnerTokens(to.clone());
        let mut owned_tokens: Vec<u64> = env
            .storage()
            .persistent()
            .get(&owner_key)
            .unwrap_or(Vec::new(&env));

        owned_tokens.push_back(token_id);
        env.storage().persistent().set(&owner_key, &owned_tokens);

        env.storage()
            .instance()
            .set(&DataKey::NextTokenId, &(token_id + 1));

        env.events()
            .publish((symbol_short!("mint"), to), (token_id, tier));

        token_id
    }

    // User redeems reward attached to NFT
    pub fn redeem(env: Env, token_id: u64, user: Address) {
        user.require_auth();

        let key = DataKey::Token(token_id);
        let mut nft: LoyaltyNFT = env.storage().persistent().get(&key).unwrap();

        if nft.owner != user {
            panic!("not token owner");
        }

        if nft.redeemed {
            panic!("already redeemed");
        }

        nft.redeemed = true;
        env.storage().persistent().set(&key, &nft);

        env.events()
            .publish((symbol_short!("redeem"), user), token_id);
    }

    // Admin updates loyalty tier and points
    pub fn update_tier(env: Env, token_id: u64, new_tier: String, new_points: u32) {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        admin.require_auth();

        let key = DataKey::Token(token_id);
        let mut nft: LoyaltyNFT = env.storage().persistent().get(&key).unwrap();

        nft.tier = new_tier.clone();
        nft.points = new_points;

        env.storage().persistent().set(&key, &nft);

        env.events()
            .publish((symbol_short!("update"), token_id), (new_tier, new_points));
    }

    // View one NFT
    pub fn get_nft(env: Env, token_id: u64) -> LoyaltyNFT {
        env.storage()
            .persistent()
            .get(&DataKey::Token(token_id))
            .unwrap()
    }

    // View all NFTs owned by a user
    pub fn get_user_nfts(env: Env, user: Address) -> Vec<u64> {
        env.storage()
            .persistent()
            .get(&DataKey::OwnerTokens(user))
            .unwrap_or(Vec::new(&env))
    }

    // View contract admin
    pub fn get_admin(env: Env) -> Address {
        env.storage().instance().get(&DataKey::Admin).unwrap()
    }
}