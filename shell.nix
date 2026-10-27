let
  # Pinned so local and CI codegen use identical tool versions (openapi-generator,
  # redocly, python, yq); a different generator version changes the generated models.
  pkgs = import (fetchTarball {
    url = "https://github.com/NixOS/nixpkgs/archive/d3b13ad75cc492690728f9307e443dec9ef9a20e.tar.gz";
    sha256 = "05r2yhqg472368kf2i0mqrvbfww94ykssphkxdxxgjxwd1jffqx4";
  }) { };
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
