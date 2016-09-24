use std::process::Command;
use std::env;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    // Get a checkout of ncurses.
    Command::new("git").args(&["submodule", "update", "--init"]).status().unwrap();

    // run the standard configure, make, make install with ncurses.
    env::set_current_dir("ncurses").ok();

    if cfg!(feature = "wide") {
        Command::new("./configure").args(&["--prefix", &out_dir, "--enable-widec"])
            .status().unwrap();
    } else {
        Command::new("./configure").args(&["--prefix", &out_dir])
            .status().unwrap();
    }

    Command::new("make").status().unwrap();
    Command::new("make").args(&["install"])
        .status().unwrap();

    println!("cargo:rustc-link-search=native={}/lib", out_dir);

    if cfg!(feature = "wide") {
        println!("cargo:rustc-link-lib=static=ncursesw");
    } else {
        println!("cargo:rustc-link-lib=static=ncurses");
    }
}
