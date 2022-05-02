# Solana Faucet 

*Objective*: build a Solana smart contract / program that can can create new mint token accounts and mint tokens to a PDA so users can request tokens for testing on devNet and testNet deployments. 

## Feature 1:

Create a permissionless `create_mint_and_vault(amount_to_mint: u64)` instruction that creates a new token account and mints the given number of tokens to a PDA address.

### Optional: 

Have a way to limit the number of times the `create_mint_and_vault(...)` instruction can be called from a single user, ie a single user's wallet can only call this twice forever.  

## Feature 2: 

Create a `request_tokens` instruction that will transfer tokens from the PDA to the requesting users wallet with a predefined number of tokens. 

### Optional: 

Create the instruction so a user's wallet can only call this instruction once per day. 

