{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    actions-nix.url = "github:nialov/actions.nix";
    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs =
    inputs@{ nixpkgs, flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [ inputs.actions-nix.flakeModules.default ];
      systems = nixpkgs.lib.systems.flakeExposed;
      perSystem =
        {
          pkgs,
          system,
          ...
        }:
        let
          rustToolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
          manifest = (pkgs.lib.importTOML ./Cargo.toml).package;
        in
        {
          formatter = pkgs.nixfmt-rfc-style;
          _module.args.pkgs = import nixpkgs {
            inherit system;
            overlays = [
              (import inputs.rust-overlay)
            ];
          };

          devShells.default = pkgs.mkShell {
            nativeBuildInputs = with pkgs; [
              rustToolchain.override
              { extensions = [ "rust-src" ]; }
              pkg-config
              libxkbcommon
            ];
          };

          packages.default =
            (pkgs.makeRustPlatform {
              rustc = rustToolchain;
              cargo = rustToolchain;
            }).buildRustPackage
              {
                pname = manifest.name;
                version = manifest.version;
                cargoLock.lockFile = ./Cargo.lock;
              };
        };
    };
}
