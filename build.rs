extern crate lalrpop;

use std::env;

fn main() {
    env::set_var("LALRPOP_LANE_TABLE", "enable");
    lalrpop::Configuration::new()
        .emit_comments(false)
        .log_verbose()
        .process_current_dir()
        .unwrap();

    // println!("cargo:rerun-if-changed=src/parser/lrparser.lalrpop");
}