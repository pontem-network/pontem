{
  inputs = {
    fenix.url = github:nix-community/fenix;
    naersk.url = github:nmattia/naersk;
    utils.url = github:numtide/flake-utils;
    move-tools.url = github:pontem-network/move-tools;

    nixpkgs.follows = "move-tools/nixpkgs";
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
        devComponents = buildComponents ++ [ "clippy-preview" "rustfmt-preview" "rust-src" ];

        devToolchain = let
            t = fenixArch.toolchainOf devToolchainV;
        in t.withComponents devComponents;

        naersk-lib =
          let
            buildToolchain = fenixArch.combine [
                ((fenixArch.toolchainOf buildToolchainV).withComponents buildComponents)
                (rustTargets.wasm32-unknown-unknown.toolchainOf buildToolchainV).toolchain
            ];
          in
            naersk.lib.${system}.override {
              cargo = buildToolchain;
              rustc = buildToolchain;
            };

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

          SKIP_WASM_BUILD = "1";
          PROTOC = "${protobuf}/bin/protoc";
          LLVM_CONFIG_PATH="${llvmPackagesR.llvm}/bin/llvm-config";
          LIBCLANG_PATH="${llvmPackagesR.libclang.lib}/lib";
          RUST_SRC_PATH = "${devToolchain}/lib/rustlib/src/rust/library/";
          ROCKSDB_LIB_DIR = "${rocksdb}/lib";
        };

        defaultPackage = naersk-lib.buildPackage (with pkgs; {
          name = "pontem-node";
          src = ./.;
          targets = [ "pontem-node" ];
          buildInputs = [
            protobuf openssl pre-commit pkgconfig
            llvmPackagesR.clang
            dove
          ];

          PROTOC = "${protobuf}/bin/protoc";
          LLVM_CONFIG_PATH="${llvmPackagesR.llvm}/bin/llvm-config";
          LIBCLANG_PATH="${llvmPackagesR.libclang.lib}/lib";
          ROCKSDB_LIB_DIR = "${rocksdb}/lib";
        });

      });

}
