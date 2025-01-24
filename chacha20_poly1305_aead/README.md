# chacha20/poly1305 AEAD

## Compilation

```sh
cargo build --release
```

## Usage

```sh
./target/release/poly1305-gen <key> <filename>
./target/release/poly1305-check <key> <filename> <tag>

./target/release/chacha20 <key file> <nonce> <input file> <output file>

./target/release/aead-wrap <key file> <nonce> <ad file> <input file> <output file>
./target/release/aead-unwrap <key file> <nonce> <ad file> <input file> <tag>
```

## Test

```sh
cargo test
```
