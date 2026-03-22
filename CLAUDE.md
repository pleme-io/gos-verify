# gos-verify

Verify GrapheneOS releases against published signing keys and check
for latest updates. BLAKE3 hashing for integrity verification.

## Usage

```bash
gos-verify devices                       # list supported devices
gos-verify latest husky                   # check latest Pixel 8 Pro release
gos-verify latest tokay --channel beta    # check beta for Pixel 9
gos-verify verify image.zip -d husky     # verify a downloaded image
gos-verify hash factory.zip              # BLAKE3 hash a file
```

## Supported Devices

All Pixel 6 through Pixel 10 series (20 devices).

## Integration

- Uses BLAKE3 for content-addressed verification (aligns with tameshi/fudajiku)
- Checks releases.grapheneos.org for latest build numbers
- Future: signature verification against published ed25519 keys
