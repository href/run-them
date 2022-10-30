# is-online

A CLI to launch a single command with many workers, serializing the output.

## Install

```bash
cargo install run-them
```

## Examples

Serve the local folder with 4 workers in Python:

```bash
$ run-them -w 4 python3 -m http.server --bind 127.0.0.1 8000
```
