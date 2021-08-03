use stdlib_fetch::{fetch, FetchConfig};
use substrate_build_script_utils::{generate_cargo_keys, rerun_if_git_head_changed};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    generate_cargo_keys();
    rerun_if_git_head_changed();

    fetch(FetchConfig {
        git_repo: "https://github.com/pontem-network/move-stdlib.git",
        rev: Some("ed4b158bdc339f7de20dc2144dfad606870b1f6f"),
        path_to_clone: "./move/stdlib",
        build_with_dove: true,
    })
    .unwrap();
}
