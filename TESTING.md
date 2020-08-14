# Testing Instruction

1. Install rust and other dependencies: https://substrate.dev/docs/en/knowledgebase/getting-started/
2. Clone repository
`git clone --branch polkadot_bridge https://github.com/vmarkushin/iroha.git && cd iroha`
4. Run iroha
`cargo run --package iroha`
6. Run substrate node
`RUST_LOG=debug cargo run --package node-template -- --dev --tmp`
8. Wait a few seconds and run tester
`cargo run --package bridge-tester`

