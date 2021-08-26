use stdlib_fetch::{fetch, FetchConfig};
use substrate_build_script_utils::{generate_cargo_keys, rerun_if_git_head_changed};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    generate_cargo_keys();
    rerun_if_git_head_changed();

    fetch(FetchConfig {
        git_repo: "https://github.com/pontem-network/move-stdlib.git",
        rev: Some("e0fa415da72b5b27f651f82a8b8a3619c5a674aa"),
        path_to_clone: "./move/stdlib",
        build_with_dove: true,
    })
    .unwrap();
}
