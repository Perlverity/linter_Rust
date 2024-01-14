use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

use regex::Regex;
use walkdir::WalkDir;

fn main() {
    println!("[Info] Linterを開始します。");

    let directory_path = env::current_exe().ok()
        .and_then(|path| path.parent().map(|path| path.to_str().unwrap_or("").to_owned()));

    lint_directory(directory_path.as_deref());

    println!("[Info] Linterが完了しました。");
}

fn lint_directory(directory_path: Option<&str>) {
    let re = Regex::new(r"fetch").unwrap();

    if let Some(directory_path) = directory_path {
        for entry in WalkDir::new(directory_path).into_iter().filter_map(|e| e.ok()) {
            if entry.path().extension().map_or(false, |e| e == "py") {
                lint_file(entry.path().to_str().unwrap(), &re);
            }
        }
    }
}

fn lint_file(filename: &str, re: &Regex) {
    let file = File::open(filename).expect("[Error] ファイルが開けませんでした。");
    let reader = BufReader::new(file);

    let mut in_for_loop = false;
    let mut for_loop_indent = 0;

    for (index, line) in reader.lines().enumerate() {
        let line = line.expect("[Error] 行が読み込めません");
        check_for_loop(&line, &mut in_for_loop, &mut for_loop_indent);

        if in_for_loop && re.is_match(&line) {
            println!("[Alert] linter {}: 'fetch' メソッドが for文の中に含まれています。", index + 1);
        }
    }
}

fn check_for_loop(line: &str, in_for_loop: &mut bool, for_loop_indent: &mut usize) {
    let current_indent = line.chars().take_while(|c| c.is_whitespace()).count();

    if line.trim_start().starts_with("for ") && line.contains(":") {
        *in_for_loop = true;
        *for_loop_indent = current_indent;
    } else if *in_for_loop && (current_indent <= *for_loop_indent && !line.trim().is_empty()) {
        *in_for_loop = false;
    }
}