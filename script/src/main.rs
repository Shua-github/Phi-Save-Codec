use multi_value_gen::parse;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::process::Command;
use walrus::ValType;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let status = Command::new("cargo")
        .args([
            "build",
            "--lib",
            "-p",
            "phi_save_codec",
            "--release",
            "--target",
            "wasm32-unknown-unknown",
        ])
        .status()?;

    if !status.success() {
        eprintln!("cargo build 失败，退出程序");
        std::process::exit(1);
    }

    let api_dir = "./app/src/api/";
    let mut funcs = HashMap::new();

    if Path::new(api_dir).exists() {
        for entry in fs::read_dir(api_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() && path.extension().map_or(false, |ext| ext == "rs") {
                if let Some(file_name) = path.file_stem() {
                    let func_name = file_name.to_string_lossy().to_string();
                    if func_name != "mod" {
                        funcs.insert(
                            format!("build_{}", func_name),
                            vec![ValType::I32, ValType::I32],
                        );
                        funcs.insert(
                            format!("parse_{}", func_name),
                            vec![ValType::I32, ValType::I32],
                        );
                    }
                }
            }
        }
    } else {
        println!("警告: 目录 {} 不存在", api_dir);
    }

    println!("找到 {} 个API函数", funcs.len());

    let wasm_file = "./target/wasm32-unknown-unknown/release/phi_save_codec.wasm";
    let wasm_bytes = fs::read(wasm_file)?;

    match parse(wasm_bytes, funcs) {
        Ok(processed_wasm) => {
            let output_dir = "./output/";
            fs::create_dir_all(output_dir)?;

            let output_path = format!("{}phi_save_codec.wasm", output_dir);
            fs::write(&output_path, processed_wasm)?;

            println!("保存到: {}", output_path);
        }
        Err(e) => {
            eprintln!("处理WASM文件时出错: {}", e);
            return Err(e.into());
        }
    }

    Ok(())
}
