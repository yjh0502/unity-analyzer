# find all dangling resources

```
cargo build --release --bin cli
./target/release/cli danglings --include '*.unity' $PROJECT_ROOT
```
