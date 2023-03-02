use std::fs::{read_to_string, OpenOptions};
use std::io::Write;

fn main() {
    println!("cargo:rerun-if-changed=src/udl");
    join_udl_files(
        &[
            "src/udl/namespace.udl",
            "src/udl/pet.udl",
            "src/udl/person.udl",
            "src/udl/test_enum.udl",
        ],
        "src/simple.udl",
    );

    uniffi::generate_scaffolding("./src/simple.udl").unwrap();
}

fn join_udl_files(paths: &[&str], output: &str) {
    let udl_file_content = paths
        .iter()
        .map(|path| {
            read_to_string(path).unwrap_or_else(|error| {
                panic!("error occurred while trying to read \"{path}\": {error}")
            })
        })
        .collect::<Vec<String>>()
        .join("\n");

    let mut udl_file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(output)
        .unwrap();

    writeln!(udl_file, "{udl_file_content}").unwrap();
}
