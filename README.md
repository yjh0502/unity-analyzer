# Build

install [rustup](https://rustup.rs/)
```
cargo build --release --bin cli
```

# How-To

find all dangling resources

```
# aggressive, print all danglings
./target/release/cli danglings -o dangling.log $PROJECT_ROOT

# mark all '*.unity' scene files and following assets as used
./target/release/cli danglings --include '*.unity' -o dangling.log $PROJECT_ROOT
```
