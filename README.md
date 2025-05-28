# Soroban Project

## Project Structure

This repository uses the recommended structure for a Soroban project:
```text
.
├── contracts
│   └── hello_world
│       ├── src
│       │   ├── lib.rs
│       │   └── test.rs
│       └── Cargo.toml
├── Cargo.toml
└── README.md
```

- admin: Muhtemelen yönetici işlevleri içerir.
- allowance: Token harcama izinleriyle ilgilidir (ERC-20 standardındaki approve ve transferFrom gibi).
- balance: Kullanıcıların token bakiyelerini yönetir.
- contract: Ana kontrat mantığını ve Soroban trait implementasyonlarını içerir.
- metadata: Token'ın adı, sembolü, ondalık sayısı gibi meta verilerini içerir.
- storage_types: Kontratın depolama için kullandığı veri türlerini tanımlar.
- test: Kontrat için testleri içerir.

- New Soroban contracts can be put in `contracts`, each in their own directory. There is already a `hello_world` contract in there to get you started.
- If you initialized this project with any other example contracts via `--with-example`, those contracts will be in the `contracts` directory as well.
- Contracts should have their own `Cargo.toml` files that rely on the top-level `Cargo.toml` workspace for their dependencies.
- Frontend libraries can be added to the top-level directory as well. If you initialized this project with a frontend template via `--frontend-template` you will have those files already included.