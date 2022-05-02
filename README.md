# Solana Faucet 

*Objective*: Build a Solana smart contract / program that can can create new mint token accounts and mint tokens to a PDA so users can request tokens for testing on devNet and testNet deployments. 

*Purpose*: During the development and testing of defi protocols it is a common practice to use a faucet to get access to test tokens. This faucet will enable users/developers (and a possible frontend at a later date) to create new SPL-token's and store the minted tokens in a PDA that can then get released to users/testers when they require them for testing. 

## Feature 1:

Create a permissionless `create_mint_and_vault(amount_to_mint: u64)` instruction that creates a new SPL-token and mints the given number of tokens to a PDA address for later release via the `request_tokens` instruction. 

### Optional: 

Have a way to limit the number of times the `create_mint_and_vault(...)` instruction can be called from a single user, ie a single user's wallet can only call this twice forever.  

## Feature 2: 

Create a `request_tokens` instruction that will transfer tokens from the PDA to the requesting users wallet with a predefined number of tokens. 

### Optional: 

Create the instruction so a user's wallet can only call this instruction once per day. 


## Contributing 

* Please submit code for review as a PR to this repo
* Please include tests for all instructions  