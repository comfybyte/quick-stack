_default:
  just --list

check:
  nix fmt
  cargo clippy

watch:
  bacon clippy

udeps:
  cargo udeps
