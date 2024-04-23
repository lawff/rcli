# rcli

### CSV

```bash
cargo run -- csv -i assets/juventus.csv
```

### GENPASS

```bash
cargo run -- genpass -l 17 --numbers=false --symbols=false
```

### BASE64

```bash
cargo run -- base64 decode -i fixtures/b64.txt -f urlsafe
```

### TEXT

```bash
cargo run -- text sign -k fixtures/blake3.txt -i fixtures/b64.txt
```

### HTTP

```bash
RUST_LOG=info cargo run -- http serve
```

## TEST

```bash
cargo nextest run
```
