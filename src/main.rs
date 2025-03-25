use chenbao_cmd::*;
use colored::*;
use std::ffi::OsStr;
use std::fs;
use std::fs::File;
use std::path::Path;
use std::process::exit;

pub fn main() {
    App::new()
        
        .about("new -- 一个快速创建文件和文件夹的程序")
        .author("https://github.com/chen-bao-x/new")
        .add_command(
            cmd!("file")
                .short_name("f")
                .about("创建文件")
                .add_example("new f filename.txt", "在当前目录创建文件")
                .add_example("new f foldername/", "在当前目录创建文件夹")
                .add_example("new f folder_1/filename.txt", "在当前目录创建 文件夹/文件")
                .add_example(
                    "new f a.txt b.txt d.txt e/in_e.txt",
                    "在创建多个文件或文件夹",
                )
                .action(Arg::PathMutiple(&|x| {
                    println!("{x:?}");
                    x.iter().for_each(|f| {
                        create_file(f);
                    });
                    x.iter().for_each(|f| create_file(f));
                })),
        )
        .add_command(
            cmd!("directory")
                .short_name("d")
                .about("创建文件夹")
                .add_example("new d directory_name", "在当前目录创建文件夹.")
                .add_example("new d folder_1 foler_2 foler_3", "在当前目录创建多个文件夹")
                .action(Arg::PathMutiple(&|x| {
                    println!("{x:?}");

                    x.iter().for_each(|f| {
                        create_dir(f);
                    });
                })),
        )
        .debug_check()
        .run();
}

fn create_file(path: &Path) {
    let mut did_parent_created = false;
    if let Some(parent) = path.parent() {
        // 如果父文件夹不存在, 则创建父文件夹.
        if !parent.exists() {
            did_parent_created = true;
            let re = fs::create_dir_all(parent);
            if let Err(err) = re {
                eprintln!("{:}", err);
                exit(0)
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
                    final_re.bright_cyan()
                } else {
                    final_re.green()
                }
            }();

            println!(
                "   file {} 创建成功 at {}{}{}",
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
                "   folder {} 创建成功 at {}{}",
                folder_name.bright_cyan(),
                parent.green(),
                path.to_string_lossy().bright_cyan(),
            );
        }
        Err(e) => eprintln!("创建文件夹 {:?} 失败: {}", folder_name, e),
    }
}
