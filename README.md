# detmatch

Match predicted object-detection boxes to ground-truth boxes and label
every box individually: true positive (TP), false positive (FP), or
false negative (FN).

Most evaluation tools summarise matches into metrics such as precision,
recall, or mAP. detmatch instead exposes the per-box result itself, so
that downstream analysis can work from the raw matches.

## Status

Early development; there is no usable functionality yet.

## Development

```sh
cargo build
cargo test
cargo fmt --check
cargo clippy --all-targets
```

## License

[MIT](LICENSE)
