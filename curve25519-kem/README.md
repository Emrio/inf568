# curve25519 kem

## Notes

This implementation does not use cryptographic libraries, it is somewhat self-contained.

The ciphertext is 64 bytes long.

## Compilation

```sh
cargo build --release
```

## Usage

```sh
./target/release/keygen <private key file>
./target/release/encaps <public key>
./target/release/decaps <private key file> <ciphertext>
```

## Test

```sh
cargo test
```
