{
  description = "yktotp-jsonapi";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable"; # We want to use packages from the binary cache
    flake-utils.url = "github:numtide/flake-utils";
    gitignore = {
      url = "github:hercules-ci/gitignore.nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { nixpkgs, flake-utils, gitignore, ... }:
  flake-utils.lib.eachSystem [ "x86_64-linux" ] (system: let
    pkgs = nixpkgs.legacyPackages.${system};
    inherit (gitignore.lib) gitignoreSource;
  in rec {
    packages.yktotp-jsonapi = pkgs.callPackage ./default.nix { src = gitignoreSource ./.; };

    legacyPackages = packages;

    defaultPackage = packages.yktotp-jsonapi;

    devShell = pkgs.mkShell {
      CARGO_INSTALL_ROOT = "${toString ./.}/.cargo";

      buildInputs = with pkgs; [ cargo rustc git pcsclite pkg-config ];
    };
  });
}
