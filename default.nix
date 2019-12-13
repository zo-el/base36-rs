{ pkgs ? import ./pkgs.nix {} }:

with pkgs;

let
  inherit (rust.packages.nightly) rustPlatform;
in

{
  base36 = buildRustPackage rustPlatform {
    name = "base36";
    src = gitignoreSource ./.;
    cargoDir = ".";
  };
}
