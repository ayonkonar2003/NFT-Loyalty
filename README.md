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

This can be used for customer rewards, event participation badges, student memberships, brand loyalty, or Web3 community engagement.

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
