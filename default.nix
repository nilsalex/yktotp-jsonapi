{ pkgs ? import <nixpkgs> {}, src ? ./. }:

pkgs.rustPlatform.buildRustPackage {
  name = "yktotp-jsonapi";

  src = src;

  cargoHash = "sha256-k4Py0ZhT0eWlvm0mnnltE6+nBa3IjirOqNHYQfpHuTk=";

  buildInputs = [ pkgs.pcsclite ];

  nativeBuildInputs = [ pkgs.pkg-config ];

  meta = {};
}
