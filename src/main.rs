use std::{
    fs::{self, File, OpenOptions},
    io::{BufRead, BufReader, Read, Write},
    path::PathBuf,
    process,
};

use clap::Parser;

#[derive(Parser)]
struct Cli {
    /// 题库: lc:leetcode  luogu:洛谷  
    platform: String,
    /// 题号
    sn: String,
}

fn main() {
    if !check_rust_src() {
        eprintln!("当前目录不是rust项目");
        return;
    }

    let args = Cli::parse();

    let mut path = PathBuf::new();
    let mut template = Vec::new();
    let mut full_name = String::new();
    let mut mod_file_path = PathBuf::from("src");
    path.push("src");

    match args.platform.as_str() {
        "lc" => {
            path.push("leetcode");
            mod_file_path.push("leetcode");
            full_name = "leetcode".to_string();
            template = include_str!("template1.rs").as_bytes().to_vec();
        }
        "luogu" => {
            path.push("luogu");
            full_name = "luogu".to_string();
            mod_file_path.push("luogu");
            template = include_str!("template2.rs").as_bytes().to_vec();
        }
        "at" => {
            path.push("atcoder");
            full_name = "atcoder".to_string();
            mod_file_path.push("atcoder");
            template = include_str!("template2.rs").as_bytes().to_vec();
        }
        _ => {
            eprintln!("这是没有预先准备的平台:{}", args.platform);
            process::exit(1);
        }
    }

    let mut file_name = full_name.clone();

    file_name.push('_');
    file_name.push_str(&args.sn);
    file_name.push_str(".rs");

    // println!("{:?}", file_name);
    path.push(file_name.clone());

    // println!("{:?}", path);

    let mut mod_append_text = full_name;
    mod_append_text.push('_');
    mod_append_text.push_str(&args.sn);

    // 在mod.rs声明
    mod_file_path.push("mod.rs");
    if !check_solution_repeat(mod_file_path.clone(), mod_append_text.clone()) {
        let mut mod_file = OpenOptions::new().append(true).open(mod_file_path).unwrap();
        writeln!(mod_file, "\nmod {};", mod_append_text).unwrap();
    } else {
        eprintln!("您已经创建过这一题");
        process::exit(1);
    }
    // 添加一行

    // 创建文件
    let mut file = File::create(path).unwrap();
    file.write_all(&template).unwrap();
}

/// 看看当前目录是否是rust项目
fn check_rust_src() -> bool {
    for entry in fs::read_dir(".").unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.file_name().unwrap() == "Cargo.toml" {
            return true;
        }
    }
    false
}
/// 看看mod.rs文件中是否已经声明了这道题目
fn check_solution_repeat(path: PathBuf, sn: String) -> bool {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();
        let words = line.split_whitespace();

        // println!("line: {:?}", line);
        match words.last() {
            Some(now_sn) => {
                let now_sn: String = now_sn.chars().filter(|x| *x != ';').collect();
                // println!("word: {}", now_sn);
                if now_sn == sn {
                    return true;
                }
            }
            None => {
                continue;
            }
        }
    }

    false
}
