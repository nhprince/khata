# Contributing to Khata

Thank you for helping. Small, focused pull requests are easiest to review.

## License of contributions

Khata is licensed under **MIT OR Apache-2.0**. By submitting a contribution you agree that it is
licensed under the same terms (like the upstream project), and that you have the right to submit it.
Do not paste code or assets you do not have the right to relicense. New third-party assets need a row
in [`ATTRIBUTION.md`](ATTRIBUTION.md), with the license file next to them.

## Before you open a pull request

```sh
cargo fmt --all
cargo clippy -p khata -p khata-ui-egui --all-targets -- -D warnings
cargo test -p khata-doc -p khata-engine -p khata-ui-egui --lib
cargo xtask assets
python3 packaging/third-party-licenses.py --check
```

## Commit messages

Commit messages become the release notes automatically, so write them for users:
"Add Heart shape to the gallery", not "fix stuff".

## Releases

Maintainers release from *Actions › Release*; see [`docs/releasing.md`](docs/releasing.md).
