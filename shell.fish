#! /usr/env fish
# A simple Fish shell script to set up environment variables for Rust development

function dev
    set -gx PATH $HOME/.cargo/bin $PATH
    set -gx CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER "clang"
    set -gx RUSTC_WRAPPER (which sccache)
    set -gx SCCACHE_CACHE_SIZE "20G"
    set -gx SCCACHE_JOBS (nproc)
    set -gx SCCACHE_DIR /dev/shm/sccache
    set -gx SCCACHE_LOCAL_CACHE_DIR /dev/shm/sccache
    set -gx CARGO_BUILD_JOBS (nproc)
    set -gx CARGO_INCREMENTAL 0
    set -gx RUSTFLAGS "-C incremental=/dev/shm/rust-incremental -C link-arg=-fuse-ld=mold -C opt-level=0"
    mkdir -p /dev/shm/rust-incremental
    set -gx NIX_ENFORCE_PURITY 0
    set -gx CARGO_HOME /dev/shm/cargo
    mkdir -p $CARGO_HOME
    sccache --start-server 
end

function clear
    # unset all the variables
    set -e CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER
    set -e RUSTC_WRAPPER
    set -e SCCACHE_CACHE_SIZE
    set -e SCCACHE_JOBS
    set -e SCCACHE_DIR
    set -e SCCACHE_LOCAL_CACHE_DIR
    set -e CARGO_BUILD_JOBS
    set -e CARGO_INCREMENTAL
    set -e RUSTFLAGS
end

function bench
    # sets up a highly optimized environment for benchmarking

    set -gx PATH $HOME/.cargo/bin $PATH
    set -gx CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER "clang"
    set -gx RUSTC_WRAPPER "sccache"
    set -gx RUSTC_WRAPPER (which sccache)
    set -gx SCCACHE_CACHE_SIZE "50G"
    set -gx SCCACHE_JOBS (nproc)
    set -gx SCCACHE_DIR /dev/shm/sccache
    set -gx SCCACHE_LOCAL_CACHE_DIR /dev/shm/sccache
    set -gx CARGO_BUILD_JOBS (nproc)
    set -gx CARGO_INCREMENTAL 0
    set -gx RUSTFLAGS "-C embed-bitcode=yes -C link-arg=-fuse-ld=mold -C opt-level=3 -C target-cpu=native -C lto=fat -C codegen-units=1 -C panic=abort -Z dylib-lto -C target-feature=+avx2,+fma,+bmi2,+adx"
    set -gx CARGO_HOME /dev/shm/cargo
    set -gx NIX_ENFORCE_PURITY 0
    mkdir -p $CARGO_HOME
    sccache --start-server
end
