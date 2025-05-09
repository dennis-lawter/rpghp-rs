{
  description = "Development flake for rpghp-rs";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, rust-overlay }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs {
        inherit system;
        overlays = [ rust-overlay.overlays.default ];
      };

      rust = pkgs.rust-bin.nightly.latest.default.override {
        extensions = [ "rust-src" ];
      };
    in
    {
      devShells.${system}.default = pkgs.mkShell {
        packages = with pkgs; [
          vscodium
          rust
          cargo-watch
          gnumake
          docker
          dbeaver-bin
          sqlx-cli
          insomnia
          tokei
          emacs
        ];

        shellHook = ''
          export PATH=${rust}/bin:$PATH
          export RUST_SRC_PATH=${rust}/lib/rustlib/src/rust/library
        '';
      };
    };
}

