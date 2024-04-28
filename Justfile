_default:
  just --list
run:
  nix run
build:
  nix build
check:
  nix flake check
fmt:
  nix fmt
watch:
  bacon
