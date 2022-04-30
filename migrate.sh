# deploy dao

near deploy \
    --wasmFile out/main.wasm \
    --initFunction "migrate" \
    --initArgs "{}" \
    --accountId ukraine-zoo.manhng.testnet