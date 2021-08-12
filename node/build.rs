use stdlib_fetch::{fetch, FetchConfig};
use substrate_build_script_utils::{generate_cargo_keys, rerun_if_git_head_changed};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    generate_cargo_keys();
    rerun_if_git_head_changed();

    fetch(FetchConfig {
        git_repo: "https://github.com/pontem-network/move-stdlib.git",
        rev: Some("7f200e70d2fc98863ecc8e004e9ee2255e97bf5a"),
        path_to_clone: "./move/stdlib",
        build_with_dove: true,
    })
    .unwrap();
}
