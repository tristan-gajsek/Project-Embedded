{
  outputs =
    { nixpkgs, ... }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
    in
    {
      devShells.${system}.default =
        with pkgs;
        mkShell rec {
          shellHook = "exec zsh";
          buildInputs = [
            expat
            fontconfig
            freetype
            gnumake
            libxml2
            pkg-config
            systemd
            xorg.libX11
            xorg.libXcursor
          ];
          nativeBuildInputs = buildInputs;
          LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;
        };
    };
}
