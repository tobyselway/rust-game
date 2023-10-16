{
  inputs = {
    nixpkgs.url = "nixpkgs/nixos-unstable";
  };
  outputs = { self, nixpkgs, ... }:
    let
      pkgs = nixpkgs.legacyPackages.x86_64-linux;
    in
    with pkgs;
  {

    devShell.x86_64-linux = pkgs.mkShell rec {
        nativeBuildInputs = [
            pkg-config
        ];
        buildInputs = [
            udev alsa-lib vulkan-loader
            xorg.libX11 xorg.libXcursor xorg.libXi xorg.libXrandr # To use the x11 feature
            libxkbcommon wayland # To use the wayland feature
            clang mold # fast linker
        ];
        LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;
    };

  };
}
