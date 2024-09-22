use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

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

        if path.exists() {
            println!("文件 {} 已存在。", first_arg);
            return;
        }

        if first_arg.ends_with("/") {
            println!("创建文件夹");
            create_dir(path);
        } else {
            println!("创建文件");
            let re = create_file(path);
            match re {
                Err(e) => {
                    print! {"path: {:?}  err: {:?}",path,e}
                }
                Ok(_) => {}
            }
        }
    }
}

fn create_file(path: &Path) -> std::io::Result<()> {
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent)?;
        }
    }
    let mut f = File::create_new(path)?;
    f.write_all(b"")?;

    Ok(())
}

fn create_dir(path: &Path) {
    match fs::create_dir_all(path) {
        Ok(_) => println!("文件夹 {:?} 创建成功。", path),
        Err(e) => println!("创建文件夹 {:?} 失败: {}", path, e),
    }
}
