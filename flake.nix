{
  description = "Development flake for rpghp-rs";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs = { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
    in
    {
      devShells.${system}.default = pkgs.mkShell
      {
        RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";

        packages = with pkgs; [
          vscodium
          rustc
          cargo
          gnumake
          docker
          dbeaver-bin
          sqlx-cli
          insomnia
        ];
      };
    };
}
