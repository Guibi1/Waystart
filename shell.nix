{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = [
    pkgs.wayland
    pkgs.wayland-protocols
    pkgs.vulkan-headers
    pkgs.vulkan-loader
    pkgs.vulkan-tools
    pkgs.gcc
    pkgs.openssl
    pkgs.pkg-config
    pkgs.libxkbcommon
    pkgs.xorg.libxcb
  ];


  shellHook = ''
    export PKG_CONFIG_PATH=${pkgs.wayland.dev}/lib/pkgconfig:$PKG_CONFIG_PATH
    export LD_LIBRARY_PATH=${pkgs.vulkan-loader}/lib:$LD_LIBRARY_PATH
    export LD_LIBRARY_PATH=${pkgs.wayland}/lib:$LD_LIBRARY_PATH
  '';
}
