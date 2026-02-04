// function defintion and body
fn main() {
    // function body
    // ! means a macro - diff than a function
    println!("hello world!");
}

//cargo is the package manager and build system of rust

// create a new project -- `cargo new prject-name`
// init a project that already exists -- `cargo init`

// Cargo.toml -- configurartion file
// [package] config related to the current package
// [dependencies] config dependency of the current package

// build code -- `cargo build` -- // creates exec file in target/debug directory
// build and run code -- `cargo run`
// check if code compiles without building -- `cargo check`  -- faster than `run`
// compile with optimizations `cargo build --release` -- takes longer than `run` creates exec in target/release

// Cargo.lock -- created automatically while building -- it has the exact dependencies of the pkg
