use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use std::path::Path;

fn main() {
    let dbc_file_dir = Path::new("resources/database/dbc/");
    let out_file_dir = Path::new("src/can/messages/");
    let mod_file_dir = Path::new("src/can/messages/mod.rs");

    if out_file_dir.exists() {
        std::fs::remove_dir_all(out_file_dir).unwrap();
    }
    std::fs::create_dir_all(out_file_dir).unwrap();

    let mod_file = File::create(mod_file_dir).unwrap();
    let mut mod_writter = BufWriter::new(mod_file);

    for entry in std::fs::read_dir(dbc_file_dir).unwrap() {
        let entry = entry.unwrap();
        let file_name = entry.file_name();
        let dbc_name = file_name.as_os_str().to_str().unwrap();
        let path = entry.path();
        let ext = path.extension().unwrap().to_str().unwrap();

        if ext == "dbc" {
            println!("cargo:rerun-if-changed={}", path.to_str().unwrap());

            let stem = path
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap()
                .replace(".", "_")
                .replace("-", "_")
                .to_lowercase();
            let out_path = out_file_dir.join(format!("{}.rs", stem));
            let file = File::create(out_path).unwrap();

            let mut dbc_gen_writer = BufWriter::new(file);
            dbc_gen_writer
                .write(b"#![allow(dead_code, unused)]\n")
                .unwrap();

            let dbc_file = std::fs::read(path).unwrap();
            dbc_codegen::codegen(dbc_name, &dbc_file, &mut dbc_gen_writer, false).unwrap();

            mod_writter
                .write(format!("pub mod {stem};\n",).as_bytes())
                .unwrap();
        }
    }

    slint_build::compile("ui/appwindow.slint").unwrap();
}
