### Advanced-Substrate ch02
---

#### 1. Tests
- commands
```bash
cargo fmt
cargo check -p pallet-kitties --tests

cargo test -p pallet-kitties -- t_create
cargo test -p pallet-kitties -- t_breed
cargo test -p pallet-kitties -- t_transfer
```

git repository: https://github.com/d2jvkpn/substrate-node-template-polkadot-v0.9.40

#### 2. "04 Kitties"
- commands
```bash
cargo build -p pallet-kitties --release

cargo test -p pallet-kitties

cargo build --release

cargo run --release -- --dev
```
- screenshots
![cargo build -p pallet-kitties --release](assets/04-01.png "cargo build -p pallet-kitties --release")

![cargo test -p pallet-kitties](assets/04-02.png "cargo test -p pallet-kitties")

![cargo build --release](assets/04-03.png "cargo build --release")

![cargo run --release -- --dev](assets/04-04a.png "cargo run --release -- --dev")

![cargo run --release -- --dev](assets/04-04b.png "cargo run --release -- --dev")

#### 3. "05 Test and assert events"
- commands
```bash
cargo test -p pallet-kitties
```
- screenshots
![cargo test -p pallet-kitties](assets/05-01.png "cargo test -p pallet-kitties")
