# poly1305

## Compilation

```sh
cargo build --release
```

## Usage

```sh
./target/release/poly1305-gen <key> <filename>
./target/release/poly1305-check <key> <filename> <tag>

./target/release/chacha20 <key file> <nonce> <input file> <output file>
```

## Test

```sh
cargo test
```
