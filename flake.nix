{
  inputs = {
    fenix.url = github:nix-community/fenix;
    naersk.url = github:nix-community/naersk;
    utils.url = github:numtide/flake-utils;
    move-tools.url = github:pontem-network/move-tools;

    nixpkgs.follows = "move-tools/nixpkgs";
    # nixpkgs.url = "/home/cab/data/cab/nixpkgs";
    naersk.follows = "move-tools/naersk";
    fenix.follows = "move-tools/fenix";
  };

  outputs = flake-args@{ self, nixpkgs, utils, naersk, fenix, move-tools, ... }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        fenixArch = fenix.packages.${system};
        rustTargets = fenixArch.targets;
        llvmPackagesR = pkgs.llvmPackages_12;

        dove = move-tools.defaultPackage."${system}";

        devToolchainV = {
          channel = "nightly";
          # date = "2021-09-13";
          # sha256 = "sha256:1f7anbyrcv4w44k2rb6939bvsq1k82bks5q1mm5fzx92k8m9518c";
          date = "2021-08-13";
          sha256 = "sha256:0g1j230zp38jdnkvw3a4q10cjf57avwpnixi6a477d1df0pxbl5n";
        };

        # Some strange wasm errors occur if not built with this old toolchain
        buildToolchainV = {
          channel = "nightly";
          date = "2021-06-28";
          sha256 = "sha256-vRBxIPRyCcLvh6egPKSHMTmxVD1E0obq71iCM0aOWZo=";
        };

        buildComponents = [ "cargo" "llvm-tools-preview" "rust-std" "rustc" "rustc-dev" ];
        devComponents = buildComponents ++ [ "clippy-preview" "rustfmt-preview" "rust-src" "rls-preview" "rust-analyzer-preview" ];

        devToolchain = let
            t = fenixArch.toolchainOf devToolchainV;
        in fenixArch.combine [
            (t.withComponents devComponents)
            (rustTargets.wasm32-unknown-unknown.toolchainOf devToolchainV).toolchain
        ];

        inherit (
            let
              buildToolchain = fenixArch.combine [
                  ((fenixArch.toolchainOf buildToolchainV).withComponents buildComponents)
                  (rustTargets.wasm32-unknown-unknown.toolchainOf buildToolchainV).toolchain
              ];
              tch = {
                cargo = buildToolchain;
                rustc = buildToolchain;
              };
            in
              {
                naersk = naersk.lib.${system}.override tch;
                rustPlatform = pkgs.makeRustPlatform tch;
              }
          ) naersk-lib rustPlatform;

      in {

        devShell =
        with pkgs; mkShell {
          buildInputs = [
            protobuf openssl pre-commit pkgconfig
            llvmPackagesR.clang
            dove
            devToolchain
          ] ++ (pkgs.lib.optionals stdenv.isDarwin [
            libiconv darwin.apple_sdk.frameworks.Security
          ]);

          PROTOC = "${protobuf}/bin/protoc";
          LLVM_CONFIG_PATH="${llvmPackagesR.llvm}/bin/llvm-config";
          LIBCLANG_PATH="${llvmPackagesR.libclang.lib}/lib";
          RUST_SRC_PATH = "${devToolchain}/lib/rustlib/src/rust/library/";
          ROCKSDB_LIB_DIR = "${rocksdb}/lib";
        };

        boop = builtins.fetchGit {
          url = "https://github.com/purestake/cumulus";
          ref = "nimbus-polkadot-v0.9.11";
          rev = "de3a9302ba0b7c64353da769615c0b19397255dd";
        };

        # altPackage = with pkgs; rustPlatform.buildRustPackage {
        #   pname = "pontem-node";
        #   version = "0.0.1";
        #   src = ./.;
        #   cargoLock = {
        #     lockFile = ./Cargo.lock;
        #     outputHashes = {
        #       "bcs-0.1.1" = nixpkgs.lib.fakeSha256;
        #       "beefy-gadget-4.0.0-dev" = nixpkgs.lib.fakeSha256;
        #       "bp-header-chain-0.1.0" = nixpkgs.lib.fakeSha256;
        #       "cumulus-client-cli-0.1.0" = nixpkgs.lib.fakeSha256;
        #     };
        #   };

        #   targets = [ "pontem-node" ];

        #   buildInputs = [
        #     protobuf openssl pre-commit pkgconfig
        #     llvmPackagesR.clang
        #     dove
        #   ];

        #   PROTOC = "${protobuf}/bin/protoc";
        #   LLVM_CONFIG_PATH="${llvmPackagesR.llvm}/bin/llvm-config";
        #   LIBCLANG_PATH="${llvmPackagesR.libclang.lib}/lib";
        #   ROCKSDB_LIB_DIR = "${rocksdb}/lib";
        # };

        # defaultPackage = with pkgs; naersk-lib.buildPackage {
        #   name = "pontem-node";
        #   src = ./.;
        #   targets = [ "pontem-node" ];
        #   buildInputs = [
        #     protobuf openssl pre-commit pkgconfig
        #     llvmPackagesR.clang
        #     dove
        #   ];

        #   PROTOC = "${protobuf}/bin/protoc";
        #   LLVM_CONFIG_PATH="${llvmPackagesR.llvm}/bin/llvm-config";
        #   LIBCLANG_PATH="${llvmPackagesR.libclang.lib}/lib";
        #   ROCKSDB_LIB_DIR = "${rocksdb}/lib";
        # };

      });

}
