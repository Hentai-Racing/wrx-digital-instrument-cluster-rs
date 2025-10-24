#!/bin/bash
set -e

APPLICATION_NAME="wrx-digital-instrument-cluster-rs"

LATEST_SDK=$(ls -1 /opt/wrx-cluster-rs | sort -V | tail -n1)
SDK_SETUP="/opt/wrx-cluster-rs/${LATEST_SDK}/environment-setup-armv8a-tdx-linux"
TARGET="aarch64-unknown-linux-gnu"
FEATURES="apalis_imx8,wayland"

# Usage: ./deploy.sh [profile]
PROFILE="${1:-debug}"

if [[ ! -f "$SDK_SETUP" ]]; then
    printf "\tERROR: Yocto SDK environment setup script not found at:\n"
    printf "\t\t$SDK_SETUP\n"
    printf "\tYou must build and install the SDK before running this script.\n"
    printf "\tTo build the SDK, run on your Yocto build machine:\n"
    printf "\t\t$ bitbake image-hr-testing-rs -c populate_sdk\n"
    printf "\tThen install the SDK as instructed by Yocto.\n"
    exit 1
fi

printf "[1/5] Sourcing Yocto SDK...\n"
printf "\tUsing SDK: $SDK_SETUP\n"
source "$SDK_SETUP"

if ! rustup target list --installed | grep -q "^$TARGET$"; then
    printf "[1b/5] Installing Rust target '$TARGET'...\n"
    rustup target add "$TARGET"
fi

printf "[2/5] Building Rust project with profile '$PROFILE' for $TARGET...\n"
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

printf "[3/5] Killing existing process on apalis-board...\n"
ssh -F "$SSH_CONFIG" -o StrictHostKeyChecking=accept-new apalis-board "killall $APPLICATION_NAME || true" > /dev/null 2>&1

printf "[4/5] Copying binary to board...\n"
scp -F "$SSH_CONFIG" -o StrictHostKeyChecking=accept-new "$BIN_PATH" apalis-board:/bin/ 2>/dev/null

printf "[4b] Verifying SHA256 hash...\n"
LOCAL_HASH=$(sha256sum "$BIN_PATH" | awk '{print $1}')
REMOTE_HASH=$(ssh -F "$SSH_CONFIG" -o StrictHostKeyChecking=accept-new apalis-board "sha256sum /bin/$APPLICATION_NAME | awk '{print \$1}'" 2>/dev/null)

if [[ "$LOCAL_HASH" != "$REMOTE_HASH" ]]; then
    printf "\tERROR: Hash mismatch after transfer!\n"
    printf "\tLocal : $LOCAL_HASH\n"
    printf "\tRemote: $REMOTE_HASH\n"
    exit 1
else
    printf "\tHash verified!\n"
fi

printf "[5/5] Starting new process on board...\n"

if [[ "$PROFILE" == "debug" ]]; then
    ssh -F "$SSH_CONFIG" -o StrictHostKeyChecking=accept-new apalis-board "export WAYLAND_DISPLAY=/run/wayland-0; /bin/$APPLICATION_NAME --cli"
else
    ssh -F "$SSH_CONFIG" -o StrictHostKeyChecking=accept-new apalis-board "export WAYLAND_DISPLAY=/run/wayland-0; nohup /bin/$APPLICATION_NAME --cli > /tmp/$APPLICATION_NAME.log 2>&1 &" > /dev/null 2>&1
fi

printf "\tDone! Deployed profile: $PROFILE\n"
