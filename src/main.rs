use colored::*;
use std::env;
use std::ffi::OsStr;
use std::fs;
use std::fs::File;
use std::path::Path;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("n 快速创建文件和文件夹");
        println!(
            r##"使用方式:
    n filename.txt
    n foldername/       # 创建文件夹时末尾一定要有 "/"
    n folder_1/folder_2/filename.txt
"##
        );
    } else {
        let first_arg = &args[1];
        let path = Path::new(first_arg);

        let absolute_path = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());

        if path.exists() {
            println!(
                "此处已有同名文件, 位于: {}",
                absolute_path.to_string_lossy().green().underline()
            );
            return;
        }

        if first_arg.ends_with("/") {
            create_dir(path);
        } else {
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
            print! {"path: {:?}  err: {:?}",path,err}
        }
        Ok(_) => {
            let filename = (path.file_name().unwrap_or(OsStr::new("示例字符串")))
                .to_str()
                .unwrap_or("");

            println!(
                "文件创建成功 {} at {}",
                filename.magenta(),
                path.to_string_lossy().green().underline()
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
            println!(
                "文件夹创建成功。\n {} at {}",
                folder_name.magenta(),
                path.to_string_lossy().green().underline()
            );
        }
        Err(e) => eprintln!("创建文件夹 {:?} 失败: {}", folder_name, e),
    }
}
