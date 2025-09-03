{
  inputs = {
    naersk.url = "github:nmattia/naersk/master";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      utils,
      naersk,
      ...
    }:
    utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };
        naersk-lib = pkgs.callPackage naersk { };
        libPath = with pkgs; lib.makeLibraryPath [ wayland vulkan-loader ];
      in
      {
        defaultPackage = naersk-lib.buildPackage {
          src = ./.;
          doCheck = true;
          pname = "waystart";
          nativeBuildInputs = [ pkgs.makeWrapper ];
          buildInputs = with pkgs; [
            libxkbcommon
          ];
          postInstall = ''
            wrapProgram "$out/bin/sixty-two" --prefix LD_LIBRARY_PATH : "${libPath}"
          '';
        };

        defaultApp = utils.lib.mkApp {
          drv = self.defaultPackage."${system}";
        };

        devShell =
          with pkgs;
          mkShell {
            buildInputs = [
              cargo
              rust-analyzer
              rustc
              rustfmt
              tokei
              libxkbcommon
            ];

            RUST_SRC_PATH = rustPlatform.rustLibSrc;
            LD_LIBRARY_PATH = libPath;
            GIT_EXTERNAL_DIFF = "${difftastic}/bin/difft";
          };
      }
    );
}
