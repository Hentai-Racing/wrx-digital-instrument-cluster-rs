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
    let rs_messages_out_dir = Path::new("src/can/messages/");
    let rs_messages_mod_dir = Path::new("src/can/messages/mod.rs");

    if rs_messages_out_dir.exists() {
        fs::remove_dir_all(rs_messages_out_dir).unwrap();
    }
    fs::create_dir_all(rs_messages_out_dir).unwrap();

    let mod_file = File::create(rs_messages_mod_dir).unwrap();
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

            let out_file_path = rs_messages_out_dir.join(format!("{stem}.rs"));
            let out_file = File::create(out_file_path.clone()).unwrap();
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

/// Generates Rust code for virtual CAN data generation
fn generate_vcan_handler() {
    use std::fs;
    use std::io::{self, Write};
    use std::path;
    use syn;

    // Read the mod.rs file
    let mod_rs_content =
        fs::read_to_string("src/can/messages/mod.rs").expect("Unable to read mod.rs");
    let mod_rs: syn::File = syn::parse_str(&mod_rs_content).expect("Unable to parse mod.rs");

    // Extract module names
    let module_names: Vec<String> = mod_rs
        .items
        .iter()
        .filter_map(|item| {
            if let syn::Item::Mod(syn::ItemMod { ident, .. }) = item {
                Some(ident.to_string())
            } else {
                None
            }
        })
        .collect();

    let mut gen_output = String::new(); // full file contents
    let mut gen_block = String::new(); // generated code
    let mut imports: Vec<String> = Vec::new(); // imported can message modules

    // Process each module
    for module_name in module_names {
        let module_path = format!("src/can/messages/{}.rs", module_name);
        imports.push(format!("use crate::can::messages::{};", module_name));

        let module_content = fs::read_to_string(&module_path).expect("Unable to read module file");
        let module_file: syn::File =
            syn::parse_str(&module_content).expect("Unable to parse module file");

        // Find the Messages enum
        let messages_enum = module_file
            .items
            .iter()
            .find_map(|item| {
                if let syn::Item::Enum(syn::ItemEnum {
                    ident, variants, ..
                }) = item
                {
                    if ident == "Messages" {
                        Some(variants)
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .expect("Messages enum not found");

        // Find all implementations for each message in messages_enum
        let impls: Vec<&syn::ItemImpl> = module_file
            .items
            .iter()
            .filter_map(|item| {
                if let syn::Item::Impl(item_impl) = item {
                    item_impl
                        .items
                        .iter()
                        .any(|impl_item| {
                            if let syn::ImplItem::Const(constant) = impl_item {
                                constant.ident == "MESSAGE_ID"
                            } else {
                                false
                            }
                        })
                        .then_some(item_impl)
                } else {
                    None
                }
            })
            .collect();

        for (variant, item_impl) in messages_enum.iter().zip(impls.iter()) {
            let signal_path = format!("{}::{}", module_name, variant.ident);

            for impl_item in &item_impl.items {
                if let syn::ImplItem::Fn(func) = impl_item {
                    if func.sig.ident == "new" {
                        let mut param_names: Vec<String> = Vec::new();
                        let mut value_expressions: Vec<String> = Vec::new();

                        for input in &func.sig.inputs {
                            if let syn::FnArg::Typed(pat_type) = input {
                                let param_name = if let syn::Pat::Ident(pat_ident) = &*pat_type.pat
                                {
                                    &pat_ident.ident
                                } else {
                                    continue;
                                };

                                let mut value_expression: String =
                                    format!("let {} = rand::thread_rng()", param_name);

                                if let syn::Type::Path(type_path) = &*pat_type.ty {
                                    if type_path.path.segments.last().unwrap().ident == "bool" {
                                        value_expression += ".gen_bool(0.5);";
                                    } else {
                                        let value_ident_path: String = format!(
                                            "{}::{}",
                                            signal_path,
                                            param_name.to_string().to_uppercase()
                                        );

                                        value_expression += &format!(
                                            ".gen_range({0}_MIN..={0}_MAX);",
                                            value_ident_path
                                        )
                                    }
                                }

                                param_names.push(param_name.to_string());
                                value_expressions.push(value_expression);
                            }
                        }

                        let frame_ident =
                            format!("{}_frame", variant.ident.to_string().to_lowercase());
                        let frame_constructor_expression: String = format!(
                            "let {} = {}::new({}).expect(\"Failed to create frame\");",
                            frame_ident,
                            signal_path,
                            param_names.join(", ")
                        );
                        let write_frame_expression: String = format!("if let Some(frame) = Frame::new({0}.id(), {0}.data()) {{socket.write_frame(frame).unwrap().await.unwrap();}}",
                            frame_ident
                        );

                        gen_block += &format!(
                            "{}\n{}\n{}\n",
                            value_expressions.join("\n"),
                            frame_constructor_expression,
                            write_frame_expression
                        );
                    }
                }
            }
        }
    }

    gen_output += "/// Generated code from build.rs::generate_vcan_handler()!\n";
    gen_output += "use embedded_can::Frame;";
    gen_output += "use rand::Rng;";
    gen_output += "use socketcan::tokio::CanSocket;";
    gen_output += "use std::sync::atomic::{AtomicBool, Ordering};";
    gen_output += "use std::sync::Arc;";
    gen_output += "use std::time::Duration;";
    gen_output += "use std::thread::sleep;";
    gen_output += &imports.join("\n");

    gen_output +=
        "pub async fn run_vcan_generator(socket: &mut CanSocket, running: Arc<AtomicBool>, simulating: Arc<AtomicBool>, delay: Duration) {";
    gen_output +=
        "    while running.load(Ordering::SeqCst) {while simulating.load(Ordering::SeqCst) {";
    gen_output += &gen_block;
    gen_output += "sleep(delay);";
    gen_output += "}}}";

    let rs_out_dir = path::Path::new("src/can/virtual_can_generator.rs");
    let rs_out_file = fs::File::create(rs_out_dir).expect("Unable to create file");
    let mut mod_writter = io::BufWriter::new(rs_out_file);

    let syn_data = syn::parse_file(gen_output.as_str()).expect("Unable to parse generated code!");
    let formatted_data = prettyplease::unparse(&syn_data);

    mod_writter
        .write(formatted_data.as_bytes())
        .expect("Failed to write to file");
}

// generates slint car data globals
fn generate_slint_car_data() {
    use std::fs;
    use std::io::{self, Write};
    use std::path;
    use syn;

    // Read the mod.rs file
    let mod_rs_content =
        fs::read_to_string("src/can/messages/mod.rs").expect("Unable to read mod.rs");
    let mod_rs: syn::File = syn::parse_str(&mod_rs_content).expect("Unable to parse mod.rs");

    // Extract module names
    let module_names: Vec<String> = mod_rs
        .items
        .iter()
        .filter_map(|item| {
            if let syn::Item::Mod(syn::ItemMod { ident, .. }) = item {
                Some(ident.to_string())
            } else {
                None
            }
        })
        .collect();

    let mut gen_output = String::new(); // full file contents
    let mut gen_block = String::new(); // generated code

    // Process each module
    for module_name in module_names {
        let module_path = format!("src/can/messages/{}.rs", module_name);

        let module_content = fs::read_to_string(&module_path).expect("Unable to read module file");
        let module_file: syn::File =
            syn::parse_str(&module_content).expect("Unable to parse module file");

        // Find the Messages enum
        let messages_enum = module_file
            .items
            .iter()
            .find_map(|item| {
                if let syn::Item::Enum(syn::ItemEnum {
                    ident, variants, ..
                }) = item
                {
                    if ident == "Messages" {
                        Some(variants)
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .expect("Messages enum not found");

        // Find all implementations for each message in messages_enum
        let impls: Vec<&syn::ItemImpl> = module_file
            .items
            .iter()
            .filter_map(|item| {
                if let syn::Item::Impl(item_impl) = item {
                    item_impl
                        .items
                        .iter()
                        .any(|impl_item| {
                            if let syn::ImplItem::Const(constant) = impl_item {
                                constant.ident == "MESSAGE_ID"
                            } else {
                                false
                            }
                        })
                        .then_some(item_impl)
                } else {
                    None
                }
            })
            .collect();

        for (variant, item_impl) in messages_enum.iter().zip(impls.iter()) {
            for impl_item in &item_impl.items {
                if let syn::ImplItem::Fn(func) = impl_item {
                    if func.sig.ident == "new" {
                        gen_block += &format!("// {}\n", variant.ident);
                        for input in &func.sig.inputs {
                            if let syn::FnArg::Typed(pat_type) = input {
                                let param_name = if let syn::Pat::Ident(pat_ident) = &*pat_type.pat
                                {
                                    &pat_ident.ident
                                } else {
                                    continue;
                                };

                                // we have to get the actual return type in case the type is an enum
                                // to do this we must search all the functions again for the current
                                // parameter and get the return type of that function
                                for impl_item in &item_impl.items {
                                    if let syn::ImplItem::Fn(func) = impl_item {
                                        if &func.sig.ident == param_name {
                                            match &func.sig.output {
                                                syn::ReturnType::Type(_, ty) => {
                                                    if let syn::Type::Path(type_path) = &**ty {
                                                        let (slint_type, init_value) =
                                                            match type_path
                                                                .path
                                                                .segments
                                                                .last()
                                                                .unwrap()
                                                                .ident
                                                                .to_string()
                                                                .as_str()
                                                            {
                                                                "bool" => {
                                                                    ("SBDataParameter", "true")
                                                                }
                                                                "u8" | "u16" | "u32" | "u64"
                                                                | "i8" | "i16" | "i32" | "i64" => {
                                                                    ("SIDataParameter", "999")
                                                                }
                                                                "f32" | "f64" => {
                                                                    ("SFDataParameter", "999.4")
                                                                }
                                                                //-// ! must implement into<SharedString> for all enum types
                                                                _ => (
                                                                    "SStrDataParameter",
                                                                    "\"?VAL?\"",
                                                                ),
                                                            };

                                                        gen_block += &format!(
                                                            "\tin property <{slint_type}> {param_name}: {{ value: {init_value}"
                                                        );
                                                        match slint_type {
                                                            "SIDataParameter"
                                                            | "SFDataParameter" => {
                                                                gen_block +=
                                                                    ", unit_str: \"?UNIT?\"";
                                                            }
                                                            _ => {}
                                                        }
                                                        gen_block += " };\n";
                                                    };
                                                }
                                                _ => {}
                                            };
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    gen_output += "// Generated code from build.rs::generate_slint_car_data()!\n";
    gen_output += "import {SFDataParameter, SIDataParameter, SBDataParameter, SStrDataParameter} from \"data_parameter.slint\";\n";
    gen_output += "export global SCarData {\n";
    gen_output += &gen_block;
    gen_output += "}";

    let rs_out_dir = path::Path::new("src/ui/data/car_data.slint");
    let rs_out_file = fs::File::create(rs_out_dir).expect("Unable to create file");
    let mut mod_writter = io::BufWriter::new(rs_out_file);

    mod_writter
        .write(gen_output.as_bytes())
        .expect("Failed to write to file");
}

fn main() {
    build_dbc();
    generate_vcan_handler();
    generate_slint_car_data();

    slint_build::compile("src/ui/main.slint").unwrap();
}
