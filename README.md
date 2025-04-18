# new

一个快速创建文件和文件夹的程序

示例:

```bash

~/test> new f hello.txt
   file hello.txt 创建成功 at ./hello.txt
~/test> new d folder       
   folder folder 创建成功 at ./folder/
~//test> new f a.txt b.txt d.txt e/in_e.txt
   file a.txt 创建成功 at ./a.txt
   file b.txt 创建成功 at ./b.txt
   folder c 创建成功 at ./c/
   file d.txt 创建成功 at ./d.txt
   file in_e.txt 创建成功 at ./e/in_e.txt
~/test>

```

是否允许用户自定义命令?
比如 自定义脚本. like this:
name String
args String
about String 
command String

name rust-proj
args String
about "创建 rust binary project"
command "cargo new "

rust-proj $1
```
cargo new $1
```
