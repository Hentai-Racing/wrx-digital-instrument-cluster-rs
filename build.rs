use std::io::Write;

fn main() {
    let dbc_file_path = "resources/database/WRX_2018.dbc";
    let out_file_path = "src/can/messages.rs";

    let dbc_file = std::fs::read(dbc_file_path).unwrap();
    println!("cargo:rerun-if-changed={}", dbc_file_path);

    let mut dbc_gen_out = std::io::BufWriter::new(std::fs::File::create(out_file_path).unwrap());
    dbc_gen_out.write(b"#![allow(dead_code, unused)]\n").unwrap();
    dbc_codegen::codegen("WRX_2018.dbc", &dbc_file, &mut dbc_gen_out, false).unwrap();

    slint_build::compile("ui/appwindow.slint").unwrap();
}
