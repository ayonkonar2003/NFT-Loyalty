# NFT Loyalty

## Project Description
NFT Loyalty is a basic smart contract built on Stellar using Soroban.  
The goal of this project is to create a blockchain-based loyalty system where businesses or communities can reward users with NFT-style loyalty records.

Instead of using traditional centralized reward points, this contract stores loyalty ownership and reward status on-chain.  
Each loyalty NFT acts like a digital membership card that can represent different customer tiers such as Bronze, Silver, or Gold.

This makes the system more transparent, secure, and user-owned.

---

## What it does
This contract allows an admin to manage loyalty NFTs for users.

With this contract:
- The admin initializes the contract
- The admin mints a loyalty NFT to a user
- Each NFT stores:
  - token ID
  - owner address
  - loyalty tier
  - reward points
  - redemption status
  - metadata URI
- Users can redeem their loyalty NFT reward
- Admin can update the user’s tier and points
- Anyone can view NFT details and check which NFTs belong to a user

This can be used for customer rewa# NFT Loyalty (Soroban Smart Contract)

## Project Description
NFT Loyalty is a decentralized loyalty reward system built on the Stellar blockchain using Soroban smart contracts.

Traditional loyalty programs store user points and rewards in centralized systems, making them less transparent and limited to a single platform. NFT Loyalty solves this by issuing blockchain-based loyalty records (NFT-style tokens) that users truly own.

Each loyalty NFT represents a membership tier such as Bronze, Silver, or Gold and can unlock rewards like discounts, premium access, or exclusive benefits.

This system brings transparency, security, and ownership to loyalty programs.

---

## What it does
This smart contract allows an admin to create and manage NFT-based loyalty cards for users on-chain.

It enables:
- Initializing the contract with an admin
- Minting loyalty NFTs to users
- Storing user tier, points, and metadata
- Tracking whether a reward has been redeemed
- Allowing users to redeem rewards
- Updating loyalty tiers and points
- Fetching NFT data and user-owned NFTs

This makes it suitable for:
- Customer reward systems
- Event participation badges
- Membership programs
- Web3 communities

---

## Features
- Admin-controlled contract initialization
- Mint loyalty NFTs to users
- Loyalty tiers (Bronze, Silver, Gold)
- On-chain storage of reward points
- Redemption tracking system
- Update tier and points functionality
- Fetch NFT details by token ID
- Fetch all NFTs owned by a user
- Event emission for mint, redeem, and updates
- Built using Soroban SDK on Stellar

---

## Smart Contract (Deployed)

🔗 View Contract:  
https://stellar.expert/explorer/testnet/contract/CBY5ZFMHKHPJJXNFFJ3UTVNYN64SK4PQSUHZM3ESK5DIK7H2GTABJ4GW :contentReference[oaicite:0]{index=0}

---

## Contract Screenshot

<img width="1919" height="900" alt="image" src="https://github.com/user-attachments/assets/c367a0f9-8149-4ecb-8b41-2b948d9a0bd5" />


> 📌 Note: If image doesn't render, take a screenshot manually and upload to your repo:rds, event participation badges, student memberships, brand loyalty, or Web3 community engagement.

---

## Features
- Admin-based contract initialization
- Mint loyalty NFTs to users
- Loyalty tiers like Bronze, Silver, and Gold
- Store reward points on-chain
- Track redemption status
- Update loyalty tier and points
- View NFT details by token ID
- View all loyalty NFTs of a user
- Event emission for initialize, mint, redeem, and update actions
- Built with Soroban SDK on Stellar

---

## Example Use Cases
- Coffee shop reward cards
- Brand loyalty memberships
- Event attendance rewards
- Student participation badges
- Exclusive access memberships
- Community achievement NFTs

---

## Future Scope
- Add transfer functionality
- Add reward expiry dates
- Add automatic tier upgrades
- Add merchant verification for redemptions
- Connect to a frontend dashboard
- Add richer metadata and reward categories

---

## Tech Stack
- Rust
- Soroban SDK
- Stellar Smart Contracts

---
