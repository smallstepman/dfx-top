# `dfx top`

Try it out:
```shell
git clone https://github.com/smallstepman/dfx-top
cd dfx-top
cargo build --release
DFX_EXTENSION_DIR="$(dfx cache show)/extensions/top"
mkdir -p $DFX_EXTENSION_DIR
cp target/release/dfx-top $DFX_EXTENSION_DIR/top
cp extension.json $DFX_EXTENSION_DIR/extension.json
dfx top
```
