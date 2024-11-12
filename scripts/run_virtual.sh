if cargo build; then
	export HR_CLUSTER_VIRTUAL=1
	export SLINT_SCALE_FACTOR=1.3
	sudo -E ./target/debug/wrx-digital-instrument-cluster-rs
fi
