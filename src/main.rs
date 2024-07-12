use std::{fs, io::{self, BufRead, BufReader}};

use bdzer::{cli_run, only_one_for_key, Cli};
use clap::Parser as _;

fn main() {
    let cli = Cli::parse();
    match cli_run(cli.base_path,cli.include_ext) {
        Ok(dup) => {
            println!();
            for (k, v) in &dup {
                println!("{}:",k);
                for i in v {
                    println!("{}", i);
                }
                println!();
            }
            
            let remove_files = only_one_for_key(&dup);
            if !remove_files.is_empty() {
                println!("是否要删除多余的文件? [y/n]:");
                let stdin = io::stdin();
                let mut reader = BufReader::new(stdin.lock());
                let mut input = String::new();
                reader.read_line(&mut input).expect("读取输入时出错");
                let user_input = input.trim().to_string();
                
                if user_input == "y" {
                    
                    for remove_file in remove_files {
                        println!("正在删除: {}", remove_file);
                        fs::remove_file(remove_file).unwrap();
                    }
                }
            }
            
        },
        Err(e) => {
            println!("{}", e);
        },
    }
}
