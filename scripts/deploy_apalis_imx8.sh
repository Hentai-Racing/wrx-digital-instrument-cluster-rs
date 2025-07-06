#!/bin/bash
set -e

APPLICATION_NAME="wrx-digital-instrument-cluster-rs"
SDK_SETUP="/opt/wrx-cluster-rs/7.3.0/environment-setup-armv8a-tdx-linux"

if [[ ! -f "$SDK_SETUP" ]]; then
    echo "ERROR: Yocto SDK environment setup script not found at:"
    echo "       $SDK_SETUP"
    echo "You must build and install the SDK before running this script."
    echo "To build the SDK, run on your Yocto build machine:"
    echo "    bitbake image-hr-testing-rs -c populate_sdk"
    echo "Then install the SDK as instructed by Yocto."
    exit 1
fi

echo "[1/5] Sourcing Yocto SDK..."
source "$SDK_SETUP"

echo "[2/5] Building Rust project for apalis_imx8..."
export CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-tdx-linux-gcc
export RUSTFLAGS="-C link-arg=--sysroot=$SDKTARGETSYSROOT"
cargo build --target aarch64-unknown-linux-gnu

SSH_CONFIG="scripts/apalis_imx8_ssh_config"

echo "[3/5] Killing existing process on apalis-board..."
ssh -F "$SSH_CONFIG" apalis-board "killall $APPLICATION_NAME || true" > /dev/null 2>&1

echo "[4/5] Copying binary to board..."
scp -F "$SSH_CONFIG" target/aarch64-unknown-linux-gnu/debug/$APPLICATION_NAME apalis-board:/bin/

echo "[5/5] Starting new process on board..."
ssh -F "$SSH_CONFIG" apalis-board  "export WAYLAND_DISPLAY=/run/wayland-0; nohup /bin/$APPLICATION_NAME >/tmp/$APPLICATION_NAME.log 2>&1 &" > /dev/null 2>&1

echo "Done!"
