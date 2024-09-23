use colored::*;
use messages::usage;
use std::ffi::OsStr;
use std::fs::File;
use std::path::Path;
use std::process::exit;
use std::{fs, io};

mod messages;

fn main() {
    let binding = usage();
    let app = Opt::clap()
        .usage(binding.as_str())
        .about("new -- 一个快速创建文件和文件夹的程序")
        .author("chen bao");
    let app2 = app.clone();

    let opt = Opt::from_clap(&app.get_matches());

    let first_arg = &opt.file;

    match first_arg {
        None => {
            let mut out = io::stdout();
            let _ = app2.write_help(&mut out);

            return;
        }
        Some(path) if path.exists() => {
            let absolute_path = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());
            let folder_name = (path.file_name().unwrap_or(OsStr::new("示例字符串")))
                .to_str()
                .unwrap_or("");

            println!(
                "此处已有同名文件 {}\n位于: {}",
                folder_name.magenta(),
                absolute_path.to_string_lossy().green().underline()
            );
        }

        Some(path) if path.to_string_lossy().ends_with("/") => {
            create_dir(path);
        }

        Some(path) => {
            create_file(path);
        }
    }
}

fn create_file(path: &Path) {
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            // 如果父文件夹不存在, 则创建父文件夹.
            let re = fs::create_dir_all(parent);
            match re {
                Err(err) => {
                    eprintln!("{:}", err);
                    exit(0)
                }
                Ok(_) => {}
            }
        }
    }

    // 创建文件
    let re = File::create_new(path);

    match re {
        Err(err) => {
            print!("创建文件时出错: ");
            eprintln! {"path: {:?}  err: {}",path,err}
        }
        Ok(_) => {
            let absolute_path = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());

            let filename = (path.file_name().unwrap_or(OsStr::new("示例字符串")))
                .to_str()
                .unwrap_or("");

            println!(
                "文件创建成功\n{} at {}",
                filename.magenta(),
                absolute_path.to_string_lossy().green().underline()
            )
        }
    };
}

fn create_dir(path: &Path) {
    let folder_name = (path.file_name().unwrap_or(OsStr::new("示例字符串")))
        .to_str()
        .unwrap_or("");

    match fs::create_dir_all(path) {
        Ok(_) => {
            let absolute_path = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());
            println!(
                "文件夹创建成功。\n{} at {}",
                folder_name.magenta(),
                absolute_path.to_string_lossy().green().underline()
            );
        }
        Err(e) => eprintln!("创建文件夹 {:?} 失败: {}", folder_name, e),
    }
}

use std::path::PathBuf;
use structopt::StructOpt;

/// A basic example
#[derive(StructOpt, Debug)]
#[structopt(name = "new")]
struct Opt {
    /// 文件名 | 文件夹名 | path/to/new/file.txt
    #[structopt(name = "filename or foldername/", parse(from_os_str))]
    file: Option<PathBuf>,
}

// command -- function
// flag   -- function | enum
// <name> -- paramter string | number | bool | Array<T> | enum
