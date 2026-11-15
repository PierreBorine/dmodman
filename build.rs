use bindgen::MacroTypeVariation::Signed;
use std::{env, fs};
use std::path::PathBuf;

#[path = "src/cli.rs"]
mod cli;

use clap::{CommandFactory, ValueEnum};
use clap_complete::{Shell, generate_to};
use cli::Cli;

const PKG_NAME: &str = env!("CARGO_PKG_NAME");

fn main() {
    println!("cargo:rustc-link-lib=archive");

    let out_dir = env::var("OUT_DIR").unwrap();

    let bindings = bindgen::Builder::default()
        .header("ffi/libarchive-wrapper.h")
        .default_macro_constant_type(Signed)
        .allowlist_var("ARCHIVE_OK")
        .allowlist_var("ARCHIVE_EOF")
        .allowlist_var("ARCHIVE_FAILED")
        .allowlist_var("ARCHIVE_FATAL")
        .allowlist_var("ARCHIVE_WARN")
        .allowlist_type("archive_entry")
        .allowlist_item("AE_IFDIR")
        .allowlist_function("archive_error_string")
        .allowlist_function("archive_entry_filetype")
        .allowlist_function("archive_entry_pathname")
        .allowlist_function("archive_entry_paths")
        .allowlist_function("archive_read_new")
        .allowlist_function("archive_read_support_format_all")
        .allowlist_function("archive_read_open_filename")
        .allowlist_function("archive_read_next_header")
        .allowlist_function("archive_read_data_block")
        .allowlist_function("archive_read_free")
        .generate()
        .unwrap();

    let out_path = PathBuf::from(&out_dir);
    bindings.write_to_file(out_path.join("bindings.rs")).expect("Couldn't save bindings.rs file.");

    // Generate shell completion
    let comp_path = out_path.join("completion");
    fs::create_dir_all(&comp_path).unwrap();
    let mut cmd = Cli::command();
    for &shell in Shell::value_variants() {
        generate_to(shell, &mut cmd, PKG_NAME, &comp_path).unwrap();
    }
}
