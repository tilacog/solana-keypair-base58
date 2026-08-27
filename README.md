# solana-keypair-base58

Convert a Solana `keypair.json` file (64-byte JSON array) to a base58 private key string.

`solana-keygen` can print the pubkey from a keypair file but intentionally has no
subcommand to print the private key. This tool fills that gap.

## Usage

```
solana-keypair-base58 <keypair.json>
```

Reads the 64-byte array, encodes it as base58, prints the string to stdout.

## Round-trip

```
solana-keygen new -o keypair.json --no-bip39-passphrase --force
solana-keypair-base58 keypair.json                          # -> base58 string
solana-keygen recover --base58-base58-input -o recovered.json --force
```