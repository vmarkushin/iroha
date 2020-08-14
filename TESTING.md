# Testing Instruction

1. Install rust: https://www.rust-lang.org/tools/install
2. Clone repository
`git clone --branch polkadot_bridge https://github.com/vmarkushin/iroha.git && cd iroha`
4. Run iroha
`cargo run --package iroha`
6. Run substrate node
`RUST_LOG=debug cargo run --package node-template -- --dev --tmp`
8. Wait a few seconds and run tester
`cargo run --package bridge-tester`

