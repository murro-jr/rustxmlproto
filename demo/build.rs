use std::process::Command;

fn main() {
    let input_file = std::env::var("INPUT_FILE");
    assert!(input_file.is_ok());
    let input_file = input_file.unwrap();

    let out_dir = std::env::var_os("PROTO_DIR").unwrap();
    let out_dir = out_dir.into_string();
    assert!(out_dir.is_ok());

    Command::new("./rustxmlproto.exe")
        .arg(&input_file)
        .arg(out_dir.unwrap())
        .output()
        .unwrap();
}
