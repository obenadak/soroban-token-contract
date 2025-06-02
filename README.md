# Soroban Token Contract (soroban-token-contract)

This project is a token contract developed for the [Soroban](https://soroban.stellar.org/) smart contract platform running on the [Stellar](https://stellar.org/) network.  
It includes standard token functionalities (transfer, balance query, approval mechanisms, etc.) as well as some admin-specific features.

## Purpose of the Project

This contract can be used as a foundation or example for the following purposes:

* Creating a custom token on Soroban.
* Learning token development practices using `soroban-sdk` and `soroban-token-sdk`.
* Providing a token infrastructure with admin-controlled features (minting, freezing accounts, clawbacks).

## Key Features

* **Token Initialization (`initialize`)**: Initializes the token with decimal places, name, symbol, and an admin address.
* **Standard Token Functions:**
  * `balance`: Queries the token balance of an address.
  * `transfer`: Transfers tokens from one address to another.
  * `approve`: Authorizes a spender to spend a specified amount of tokens.
  * `allowance`: Queries how many tokens a spender is allowed to use on behalf of an owner.
  * `transfer_from`: Transfers an approved amount from one address to another.
  * `burn`: Burns (destroys) tokens from a specific address.
  * `burn_from`: Burns an approved amount of tokens from a specific address.
* **Admin Functions:**
  * `mint`: Mints new tokens to a specific address (admin only).
  * `set_admin`: Changes the contract's admin (admin only).
  * `set_authorized`: Sets whether an account can transfer tokens (freeze/unfreeze, admin only).
  * `clawback`: Retrieves tokens from a specific account (admin only).
* **Metadata Functions:**
  * `decimals`: Returns the number of decimal places of the token.
  * `name`: Returns the name of the token.
  * `symbol`: Returns the symbol of the token.

### Build the Contract

```bash
soroban contract build
```

*The code in this project has been interpreted with the help of AI.*