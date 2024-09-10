/// Generates Rust code from dbc files in resources/database/dbc/
///
/// The generated code is placed in src/can/messages/
fn build_dbc() {
    use dbc_codegen::{self, Config};
    use std::fs::{self, File};
    use std::io::BufWriter;
    use std::io::Write;
    use std::path::Path;

    let dbc_file_dir = Path::new("resources/database/dbc/");
    let out_file_dir = Path::new("src/can/messages/");
    let mod_file_dir = Path::new("src/can/messages/mod.rs");

    if out_file_dir.exists() {
        fs::remove_dir_all(out_file_dir).unwrap();
    }
    fs::create_dir_all(out_file_dir).unwrap();

    let mod_file = File::create(mod_file_dir).unwrap();
    let mut mod_writter = BufWriter::new(mod_file);

    for entry in fs::read_dir(dbc_file_dir).unwrap() {
        let entry = entry.unwrap();
        let file_name = entry.file_name();
        let dbc_name = file_name.as_os_str().to_str().unwrap();
        let entry_path = entry.path();
        let entry_ext = entry_path.extension().unwrap().to_str().unwrap();

        if entry_ext == "dbc" {
            println!("cargo:rerun-if-changed={}", entry_path.to_str().unwrap());

            let stem = entry_path
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap()
                .replace(".", "_")
                .replace("-", "_")
                .to_lowercase();

            let out_path = out_file_dir.join(format!("{stem}.rs"));
            let out_file = File::create(out_path).unwrap();
            let dbc_file = fs::read(entry_path).unwrap();

            let config = Config::builder()
                .dbc_name(dbc_name)
                .dbc_content(&dbc_file)
                .allow_dead_code(true)
                .build();

            dbc_codegen::codegen(config, &mut BufWriter::new(out_file))
                .expect("Failed to generate dbc code");

            mod_writter
                .write(format!("pub mod {stem};\n").as_bytes())
                .unwrap();
        }
    }
}

fn main() {
    build_dbc();
    slint_build::compile("ui/main.slint").unwrap();
}
