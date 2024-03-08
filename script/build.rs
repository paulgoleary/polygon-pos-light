use sp1_helper::build_program;

fn main() {
    // This is a hack to make sure that the build script is re-run when the milestone dependency is changed.
    println!("cargo:rerun-if-changed=../milestone");
    build_program("../program")
}
