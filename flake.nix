{
  description = "A start menu for Wayland-based window managers";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    crane.url = "github:ipetkov/crane";
  };

  outputs =
    {
      self,
      nixpkgs,
      crane,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        craneLib = crane.mkLib pkgs;
        libPath = with pkgs; lib.makeLibraryPath [ wayland vulkan-loader ];

        commonArgs = {
          src = craneLib.cleanCargoSource ./.;
          strictDeps = true;

          buildInputs = with pkgs; [ makeWrapper libxkbcommon ];
        };

        waystart = craneLib.buildPackage (
          commonArgs
          // {
            cargoArtifacts = craneLib.buildDepsOnly commonArgs;
            postInstall = ''
              wrapProgram "$out/bin/waystart" --prefix LD_LIBRARY_PATH : "${libPath}"
            '';
          }
        );
      in
      {
        checks = { inherit waystart; };
        packages.default = waystart;
        apps.default = flake-utils.lib.mkApp {
          drv = waystart;
        };

        devShells.default = craneLib.devShell {
          checks = self.checks.${system};

          packages = with pkgs; [ libxkbcommon ];
          LD_LIBRARY_PATH = libPath;
        };
      }
    );
}
