use std::process::Command;

extern crate mktemp;
use convert_case::{Case, Casing};
use mktemp::Temp;
use std::{fs, fs::File, io, io::Write};

fn prepend_file(data: String, file_path: String) -> io::Result<()> {
    // Create a temporary file
    let tmp_path = format!("{}_tmp", file_path);
    let mut tmp = File::create(&tmp_path)?;
    // Open source file for reading
    let mut src = File::open(&file_path)?;
    // Write the data to prepend
    tmp.write_all(data.as_bytes())?;
    // Copy the rest of the source file
    io::copy(&mut src, &mut tmp)?;
    fs::remove_file(&file_path)?;
    fs::rename(&tmp_path, &file_path)?;
    Ok(())
}

fn main() {
    let input_file = std::env::var("INPUT_FILE");
    assert!(input_file.is_ok());
    let input_file = input_file.unwrap();

    let out_dir = std::env::var_os("OUT_DIR").unwrap();
    let out_dir = out_dir.into_string();
    assert!(out_dir.is_ok());

    let output = Command::new("./rustxmlproto.exe")
        .arg(&input_file)
        .arg(out_dir.unwrap())
        .output();

    if let Ok(_output) = output {
        let output_file = "./src/main.rs";
        let input_file: Vec<&str> = input_file.split("/").collect();
        let input_file = input_file.last().unwrap();
        let input_file: Vec<&str> = input_file.split(".").collect();
        let input_file = input_file.first().unwrap();
        let input_file = format!("{}.rs", input_file.to_case(Case::Snake));

        let data = format!(
            "include!(concat!(env!(\"OUT_DIR\"), \"./{}\"));\n",
            input_file
        );
        prepend_file(data.to_string(), output_file.to_string());
    }
}
