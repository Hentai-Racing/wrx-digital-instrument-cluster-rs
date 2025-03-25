if cargo build; then
	sudo -E ./target/debug/wrx-digital-instrument-cluster-rs --virtual
fi
