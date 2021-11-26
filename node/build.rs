use stdlib_fetch::{fetch, FetchConfig};
use substrate_build_script_utils::{generate_cargo_keys, rerun_if_git_head_changed};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    generate_cargo_keys();
    rerun_if_git_head_changed();

    fetch(FetchConfig {
        git_repo: "https://github.com/pontem-network/move-stdlib.git",
        rev: Some("c53d57d3993b9ee4b9dc63defc26527bf3942d74"),
        path_to_clone: "./move/stdlib",
        build_with_dove: true,
    })
    .unwrap();
}
