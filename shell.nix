let
  pkgs = import <nixpkgs> { };
in
pkgs.mkShell {
  packages = with pkgs; [
    # rustup alone, so rust-toolchain.toml picks the compiler
    rustup
    cargo-audit
    # make codegen
    python3
    yq # python yq: `yq '.'` must emit JSON
    redocly
    openapi-generator-cli
  ];
}
