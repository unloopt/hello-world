rustupHomeDir="${PROJECT_DIR}/.rustup"
mkdir -p "${rustupHomeDir}"
export RUSTUP_HOME="${rustupHomeDir}"

rustup default stable
rustup target add wasm32-wasi