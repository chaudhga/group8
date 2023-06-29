Fractional NFT Program
====
This program allows you to lock a metaplex NFT and issue SPL tokens that represent fractional ownership of the locked NFT. The SPL-tokens can be traded or held by users, and can be redeemed for the underlying NFT by transferring them back to the program's multisig account for the whole supply of SPL-tokens.
## Detailed Description
The Fractional NFT program allows you to lock a `metaplex` NFT and issue SPL tokens that represent fractional ownership of the locked NFT. You can then distribute the tokens to users, who can trade or hold them as they wish.

When you lock the `metaplex` NFT in the program and issue SPL tokens to represent fractional ownership of the NFT, ownership of the NFT is transferred to a specific account controlled by your program. This account is called the program's "PDA", or program-derived address.

The program is controlled by a "multisig" account that has the authority to sign transactions on behalf of multiple parties. The multisig holds the SPL tokens that represent fractional ownership of the NFT, and can transfer them to users who want to acquire fractional ownership of the NFT.

Users can acquire fractional ownership of the locked NFT by purchasing the SPL tokens from the multisig. When a user wants to redeem their SPL tokens for the underlying NFT, they can transfer all of their tokens back to the multisig, and the multisig can then transfer ownership of the NFT to the user.

Note that the program does not have any built-in functionality for transferring ownership of the NFT to specific users. This functionality would need to be implemented separately, either by modifying the program to allow for transfers of ownership, or by using an external mechanism (such as a DEX) to allow users to exchange their SPL tokens for the underlying NFT.
## Prerequisites
* Anchor CLI with dependencies (<https://www.anchor-lang.com/docs/installation>)
## Usage
1) Clone this repository and navigate to the project directory.
2) Build and deploy the program:
```
anchor build --verifiable
solana program deploy target/deploy/fractional_nft.so --keypair <path_to_keypair>
```
3) Create the program's accounts:
```
solana account create --keypair <path_to_keypair> --space 165 <program_id>
solana account create --keypair <path_to_keypair> --space 165 <nft_account_pubkey>
solana account create --keypair <path_to_keypair> --space 165 <nft_owner_pubkey>
solana account create --keypair <path_to_keypair> --space 165 <token_mint_pubkey>
solana account create --keypair <path_to_keypair> --space 165 <token_account_pubkey>
solana account create --keypair <path_to_keypair> --space 165 <multisig_pubkey>
```
4) Initialize the program's accounts:
```
anchor init-fractional-nft --nft-program-id <nft_program_id> --nft-id <nft_id> --from <from_pubkey> --token-amount <token_amount> --program-id <program_id> --nft-account <nft_account_pubkey> --nft-owner <nft_owner_pubkey> --token-mint <token_mint_pubkey> --token-account <token_account_pubkey> --multisig <multisig_pubkey> --token-program-id <token_program_id>
```
