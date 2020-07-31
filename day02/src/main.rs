use std::fs;
use std::io;
use std::io::prelude::*;

fn main() {
    // 简单读取文件内容
    let content = fs::read_to_string("./src/test.txt").unwrap();

    println!("Content:{}", content);

    // 简单写入文件内容
    fs::write("./src/test.txt", "Hello Write").unwrap();

    // 简单创建文件夹
    let result = fs::create_dir("./src/test_dir");
    // 错误处理
    match result {
        Ok(_) => {
            println!("文件夹创建成功");
        }
        Err(err) => match err.kind() {
            io::ErrorKind::AlreadyExists => {
                println!("文件夹已经存在");
            }
            _ => {
                println!("文件夹创建失败，{}", err);
            }
        },
    }

    // 简单创建整个路径上所有不存在的文件夹
    fs::create_dir_all("./src/t1/t2/t3").unwrap();

    // 简单读取文件夹
    let dir_result = fs::read_dir("./src").unwrap();
    for entry in dir_result {
        let entry = entry.unwrap();

        println!("Path: {:?}", entry.path());
    }

    // File对象操作读取文件
    let mut file = fs::File::open("./src/test.txt").unwrap();
    let mut cc = String::new();

    file.read_to_string(&mut cc).unwrap();

    println!("cc: {}", cc);

    // File对象操作写入文件
    let mut w_file = fs::OpenOptions::new()
        .write(true)
        .open("./src/test.txt")
        .unwrap();

    w_file.write_all(b"Hello World").unwrap();

    // File对象操作追加写入文件
    let mut w_a_file = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open("./src/test.txt")
        .unwrap();

    w_a_file.write_all(b"\nHello World Line 2").unwrap();
}
