#!/bin/bash
set -e

APPLICATION_NAME="wrx-digital-instrument-cluster-rs"
SDK_SETUP="/opt/wrx-cluster-rs/7.3.0/environment-setup-armv8a-tdx-linux"
TARGET="aarch64-unknown-linux-gnu"
FEATURES="apalis_imx8,wayland"

# Usage: ./deploy.sh [profile]
PROFILE="${1:-debug}"

if [[ ! -f "$SDK_SETUP" ]]; then
    echo "ERROR: Yocto SDK environment setup script not found at:"
    echo "  $SDK_SETUP"
    echo "You must build and install the SDK before running this script."
    echo "To build the SDK, run on your Yocto build machine:"
    echo "  bitbake image-hr-testing-rs -c populate_sdk"
    echo "Then install the SDK as instructed by Yocto."
    exit 1
fi

echo "[1/5] Sourcing Yocto SDK..."
source "$SDK_SETUP"

echo "[2/5] Building Rust project with profile '$PROFILE' for $TARGET..."
export CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-tdx-linux-gcc
export RUSTFLAGS="-C link-arg=--sysroot=$SDKTARGETSYSROOT"
BUILD_EXTRA_FLAGS="--no-default-features --features $FEATURES"

if [[ "$PROFILE" == "debug" ]]; then
    cargo build --target $TARGET $BUILD_EXTRA_FLAGS
    PROFILE_DIR="debug"
elif [[ "$PROFILE" == "release" ]]; then
    cargo build --target $TARGET --release $BUILD_EXTRA_FLAGS
    PROFILE_DIR="release"
else
    cargo build --target $TARGET --profile $PROFILE $BUILD_EXTRA_FLAGS
    PROFILE_DIR="$PROFILE"
fi

SSH_CONFIG="scripts/apalis_imx8_ssh_config"
BIN_PATH="target/$TARGET/$PROFILE_DIR/$APPLICATION_NAME"

echo "[3/5] Killing existing process on apalis-board..."
ssh -F "$SSH_CONFIG" -o StrictHostKeyChecking=accept-new apalis-board "killall $APPLICATION_NAME || true" > /dev/null 2>&1

echo "[4/5] Copying binary to board..."
scp -F "$SSH_CONFIG" -o StrictHostKeyChecking=accept-new "$BIN_PATH" apalis-board:/bin/ 2>/dev/null

echo "[4b] Verifying SHA256 hash..."
LOCAL_HASH=$(sha256sum "$BIN_PATH" | awk '{print $1}')
REMOTE_HASH=$(ssh -F "$SSH_CONFIG" -o StrictHostKeyChecking=accept-new apalis-board "sha256sum /bin/$APPLICATION_NAME | awk '{print \$1}'" 2>/dev/null)

if [[ "$LOCAL_HASH" != "$REMOTE_HASH" ]]; then
    echo "ERROR: Hash mismatch after transfer!"
    echo "Local : $LOCAL_HASH"
    echo "Remote: $REMOTE_HASH"
    exit 1
else
    echo "Hash verified!"
fi

echo "[5/5] Starting new process on board..."

if [[ "$PROFILE" == "debug" ]]; then
    ssh -F "$SSH_CONFIG" -o StrictHostKeyChecking=accept-new apalis-board "export WAYLAND_DISPLAY=/run/wayland-0; /bin/$APPLICATION_NAME"
else
    ssh -F "$SSH_CONFIG" -o StrictHostKeyChecking=accept-new apalis-board "export WAYLAND_DISPLAY=/run/wayland-0; nohup /bin/$APPLICATION_NAME > /tmp/$APPLICATION_NAME.log 2>&1 &" > /dev/null 2>&1
fi

echo "Done! Deployed profile: $PROFILE"
