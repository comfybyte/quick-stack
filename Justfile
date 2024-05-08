_default:
  just --list

check:
  cargo clippy

watch:
  bacon clippy

udeps:
  cargo udeps
