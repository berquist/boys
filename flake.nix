{
  description = "A Rust library for calculation of the Boys function";

  # Flake inputs
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay"; # A helper for Rust + Nix
    systems.url = "github:nix-systems/default";
    flake-utils = {
      url = "github:numtide/flake-utils";
      inputs.systems.follows = "systems";
    };
  };

  # Flake outputs
  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
      flake-utils,
      ...
    }:
    let
      # Overlays enable you to customize the Nixpkgs attribute set
      overlays = [
        # Makes a `rust-bin` attribute available in Nixpkgs
        (import rust-overlay)
        # Provides a `rustToolchain` attribute for Nixpkgs that we can use to
        # create a Rust environment
        (self: super: {
          rustToolchain = super.rust-bin.stable.latest.default;
        })
      ];
    in
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit overlays system; };
      in
      {
        # Development environment output
        devShells.default = pkgs.mkShell {
          # The Nix packages provided in the environment
          packages =
            (with pkgs; [
              gsl
              # The package provided by our custom overlay. Includes cargo, Clippy, cargo-fmt,
              # rustdoc, rustfmt, and other tools.
              rustToolchain
            ])
            ++ pkgs.lib.optionals pkgs.stdenv.hostPlatform.isDarwin (with pkgs; [ libiconv ]);
        };
      }
    );
}
