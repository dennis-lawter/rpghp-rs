{
  description = "Development flake for rpghp-rs";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay"; # Add rust-overlay for pinned Rust
  };

  outputs = { self, nixpkgs, rust-overlay }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs {
        inherit system;
        overlays = [ rust-overlay.overlays.default ]; # Apply Rust overlay
      };

      rust = pkgs.rust-bin.nightly.latest.default; # Pinned Rust Nightly
    in
    {
      devShells.${system}.default = pkgs.mkShell {
        packages = with pkgs; [
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

