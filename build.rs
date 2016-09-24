use std::process::Command;
use std::env;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    Command::new("git").args(&["submodule", "update", "--init"]).status().unwrap();

    // WIP.
    // run the standard configure, make, make install with ncurses.
    env::set_current_dir("ncurses").ok();

    Command::new("./configure").args(&["--prefix", &out_dir, "--enable-widec"])
        .status().unwrap();

    Command::new("make").status().unwrap();
    Command::new("make").args(&["install"])
        .status().unwrap();

    println!("cargo:rustc-link-search=native={}/lib", out_dir);
    println!("cargo:rustc-link-lib=static=ncursesw");
}
