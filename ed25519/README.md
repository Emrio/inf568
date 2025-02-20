# ed25519

## Reference

[rfc8032](https://datatracker.ietf.org/doc/html/rfc8032)

## Compilation

```bash
cargo build --release
```

## Usage

```bash
./target/release/keygen <prefix>
./target/release/sign <prefix> <data file> <output signature file>
./target/release/verify <public key file> <data file> <signature file>
```

## Test

```bash
cargo test
```
