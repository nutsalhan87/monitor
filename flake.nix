{
  outputs = { self, nixpkgs }:
    let pkgs = nixpkgs.legacyPackages.x86_64-linux;

    in {
      devShell.x86_64-linux = pkgs.mkShell {
        buildInputs = with pkgs; [
          pkg-config
          fontconfig
          freetype
        ];

        shellHook = ''
          echo 'Program need perf tool for -p key'
        '';
      };
    };
}
