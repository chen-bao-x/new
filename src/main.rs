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

    let pathes = &opt.files;

    match pathes {
        p if p.is_empty() => {
            let mut out = io::stdout();
            let _ = app2.write_help(&mut out);

            return;
        }
        p => {
            p.iter().for_each(create_one);
        }
    }
}

fn create_file(path: &Path) {
    let mut did_parent_created = false;
    if let Some(parent) = path.parent() {
        // 如果父文件夹不存在, 则创建父文件夹.
        if !parent.exists() {
            did_parent_created = true;
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

            let bbb = absolute_path.to_str().unwrap_or("");

            let parent = bbb.trim_end_matches(path.to_str().unwrap_or(""));

            let filename = (path.file_name().unwrap_or(OsStr::new("示例字符串")))
                .to_str()
                .unwrap_or("");

            // 标记出新创建的 parent folders
            let created_parent = || -> ColoredString {
                let re = String::new()
                    + path
                        .parent()
                        .unwrap_or(Path::new(""))
                        .to_str()
                        .unwrap_or("");

                if re.is_empty() {
                    return ColoredString::from("");
                }

                // path.parent().unwrap().to_str() 末尾没有 '/',
                // 所以在此处手动加上.
                let final_re = re + "/";
                if did_parent_created {
                    return final_re.bright_cyan();
                } else {
                    return final_re.green();
                }
            }();

            println!(
                "   {} {} 创建成功 at {}{}{}",
                "file",
                filename.bright_yellow(),
                parent.green(),
                created_parent,
                filename.bright_yellow(),
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

            let abs_str = absolute_path.to_str().unwrap_or("");

            // 标记出 之前就存在的, 并不是我们创建的.
            let parent =
                abs_str.trim_end_matches(path.to_str().unwrap_or("").trim_end_matches("/"));

            println!(
                "   {} {} 创建成功 at {}{}",
                "folder",
                folder_name.bright_cyan(),
                parent.green(),
                path.to_string_lossy().bright_cyan(),
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
    #[structopt(parse(from_os_str))]
    files: Vec<PathBuf>,
}

// command -- function
// flag   -- function | enum
// <name> -- paramter string | number | bool | Array<T> | enum

fn create_one(pathbuf: &PathBuf) {
    match pathbuf {
        path if path.exists() => {
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

        path if path.to_string_lossy().ends_with("/") => {
            create_dir(path);
        }

        path => {
            create_file(path);
        }
    }
}
