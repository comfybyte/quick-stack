{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    naersk.url = "github:nix-community/naersk";
    parts.url = "github:hercules-ci/flake-parts";
    rust = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs = inputs@{ nixpkgs, parts, rust, naersk, ... }:
    parts.lib.mkFlake { inherit inputs; } {
      systems = [ "x86_64-linux" ];

      perSystem = { system, ... }:
        let
          overlays = [ (import rust) ];
          pkgs = import nixpkgs { inherit system overlays; };
          rustToolchain = pkgs.pkgsBuildHost.rust-bin.fromRustupToolchainFile
            ./rust-toolchain.toml;
          naersk-lib = pkgs.callPackage naersk { };
        in {
          devShells.default = pkgs.mkShell {
            buildInputs = with pkgs; [ rustToolchain just bacon cargo-udeps ];
          };

          packages.default = naersk-lib.buildPackage { src = ./.; };
        };
    };
}
