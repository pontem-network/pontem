use stdlib_fetch::{fetch, FetchConfig};
use substrate_build_script_utils::{generate_cargo_keys, rerun_if_git_head_changed};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    generate_cargo_keys();
    rerun_if_git_head_changed();

    fetch(FetchConfig {
        git_repo: "https://github.com/pontem-network/move-stdlib",
        rev: Some("ccd25dfc85c812f56b4a7120bce793edd5f19064"),
        path_to_clone: "./move/move-stdlib",
        build_with_dove: true,
    })
    .unwrap();

    fetch(FetchConfig {
        git_repo: "https://github.com/pontem-network/pont-stdlib.git",
        rev: Some("0702cdf5d696bc50b366e04de1b59ccc3d904032"),
        path_to_clone: "./move/pont-stdlib",
        build_with_dove: true,
    })
    .unwrap();
}
