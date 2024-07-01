export HR_CLUSTER_VIRTUAL=1
cargo build
sudo -E ./target/debug/wrx-digital-instrument-cluster-rs
