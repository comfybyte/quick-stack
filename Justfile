_default:
  just --list
check:
  nix fmt
  nix flake check
watch:
  bacon clippy
udeps:
  cargo udeps
