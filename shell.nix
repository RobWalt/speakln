# shell.nix

{ pkgs ? import <nixos> { } }:
with pkgs; mkShell {
  nativeBuildInputs =
    [
      pkgconfig
      clang
      libclang
      lld
    ];
  buildInputs =
    [
      speechd
    ];
  shellHook =
    ''export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${pkgs.lib.makeLibraryPath [ libclang speechd ]}"'';
}
