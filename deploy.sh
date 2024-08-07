cargo build --release --target wasm32-unknown-unknown --package test_backend

candid-extractor target/wasm32-unknown-unknown/release/test_backend.wasm > src/test_backend/test_backend.did

dfx deploy test_backend