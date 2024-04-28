{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    treefmt-nix.url = "github:numtide/treefmt-nix";
    utils.url = "github:numtide/flake-utils";
    parts.url = "github:hercules-ci/flake-parts";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs =
    inputs@{ nixpkgs, parts, utils, rust-overlay, crane, treefmt-nix, ... }:
    parts.lib.mkFlake { inherit inputs; } {
      systems = utils.lib.defaultSystems;

      perSystem = { pkgs, system, ... }:
        let
          overlays = [ (import rust-overlay) ];
          pname = "quick-stack";
          rustToolchain = pkgs.pkgsBuildHost.rust-bin.fromRustupToolchainFile
            ./rust-toolchain.toml;
          craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;
          src = craneLib.cleanCargoSource (craneLib.path ./.);
          commonArgs = {
            inherit pname src;
            strictDeps = true;
            version = "0.1";
          };
          cargoArtifacts = craneLib.buildDepsOnly commonArgs;
          bin =
            craneLib.buildPackage (commonArgs // { inherit cargoArtifacts; });

          treeFmtEval = treefmt-nix.lib.evalModule pkgs ./treefmt.nix;
        in {
          _module.args.pkgs = import nixpkgs { inherit system overlays; };

          devShells.default = craneLib.devShell {
            inputsFrom = [ bin ];
            packages = with pkgs; [ just bacon ];
          };

          # `nix build`.
          packages = {
            inherit bin;
            default = bin;
          };

          # `nix run`.
          apps.default = utils.lib.mkApp { drv = bin; };

          # `nix flake check`.
          checks = {
            inherit bin;
            fmt = craneLib.cargoFmt { inherit src; };
            clippy =
              craneLib.cargoClippy (commonArgs // { inherit cargoArtifacts; });
          };
          # `nix fmt`
          formatter = treeFmtEval.config.build.wrapper;
        };
    };
}
