# Contributing guide

Thank you for investing your time in contributing to Comodor CLI.

## Development

The development environment is managed by [Nix](https://nixos.org/download.html).
Running `nix-shell` will spawn a shell with everything you need to get started with the lib.

If you do not want to use Nix, you can either use [rustup](https://rust-lang.github.io/rustup/index.html):

```text
rustup update
```

or install manually the following dependencies:

- [cargo](https://doc.rust-lang.org/cargo/)
- [rustc](https://doc.rust-lang.org/stable/rustc/platform-support.html)

## Build

```text
cargo build
```

You can disable default [features](https://doc.rust-lang.org/cargo/reference/features.html) with `--no-default-features` and enable features with `--features feat1,feat2,feat3`.

Finally, you can build a release with `--release`:

```text
cargo build --no-default-features --features server,notify --release
```

## Commit style

Starting from the `v1.0.0`, Comodor CLI tries to adopt the [conventional commits specification](https://www.conventionalcommits.org/en/v1.0.0/#summary).