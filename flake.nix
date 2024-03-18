{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };
  };
  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem
      (system:
        let
          overlays = [ (import rust-overlay) ];
          pkgs = import nixpkgs {
            inherit system overlays;
          };
          rustEnv = pkgs.rust-bin.stable.latest.default.override {
            extensions = [ "rust-analyzer" "clippy" ];
          };
        in
        with pkgs;
        {
          devShells.default = mkShell {
            buildInputs = [ rustEnv pkg-config pcsclite ];
          };
        }
      );
}
