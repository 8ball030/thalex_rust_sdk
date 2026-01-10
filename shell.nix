let
  pkgs = import <nixpkgs> { };
in
pkgs.mkShell {
  packages = with pkgs; [
    rustup
    cargo
    # node stuff
    # node stuff
    nodejs_22
    corepack_22
    typeshare
    openapi-generator-cli
    python312
    redocly
    linuxPackages.perf
    sccache
    mold
    clang

  ];
  nativeBuildInputs =
    with pkgs;
    [
      # Rust toolchain
      rustc
      cargo
      rustup
      pkg-config
    ];
  buildInputs =
    with pkgs;
    [ 
      openssl
      # Add these dependencies for bundling
    ]
    ++ lib.optionals stdenv.hostPlatform.isLinux [
      # Required for most applications
      glib-networking
    ]
    ++ lib.optionals stdenv.hostPlatform.isDarwin [
      darwin.apple_sdk.frameworks.WebKit
    ];
  
  shellHook = ''
    # Rust toolchain
    rustup default 1.88.0
    rustup component add rust-src

    # Paths
    export PATH=$HOME/.cargo/bin:$PATH

    # Linker & optimizer
    export CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER="clang"

    # sccache config (super fast in-RAM caching)
    export RUSTC_WRAPPER="sccache"
    export RUSTC_WRAPPER="${pkgs.sccache}/bin/sccache"

    export SCCACHE_CACHE_SIZE="20G"
    export SCCACHE_JOBS=$(nproc)
    # sscache --start-server

    # Cargo parallelism
    export CARGO_BUILD_JOBS=$(nproc)
    export CARGO_INCREMENTAL=0

    export RUSTFLAGS="-C link-arg=-fuse-ld=mold -C opt-level=0"
    export CARGO_HOME=/dev/shm/cargo
    mkdir -p $CARGO_HOME
    sccache --start-server | true

	
  '';
}
