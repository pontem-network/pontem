use stdlib_fetch::{fetch, FetchConfig};
use substrate_build_script_utils::{generate_cargo_keys, rerun_if_git_head_changed};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    generate_cargo_keys();
    rerun_if_git_head_changed();

    fetch(FetchConfig {
        git_repo: "https://github.com/pontem-network/move-stdlib.git",
        rev: Some("34cedad7436f7dad6c9a521f3f6199324737d69f"),
        path_to_clone: "./move/stdlib",
        build_with_dove: true,
    })
    .unwrap();
}
