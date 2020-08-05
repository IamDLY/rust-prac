use std::fs;
use std::io::prelude::*;
use std::io::stdin;
use std::process::Command;

fn main() {
    let mut read_dir = String::new();

    println!("请输入合并视频目录：");

    stdin().read_line(&mut read_dir).expect("读取输入数据异常");

    println!("dir:{}", read_dir.trim().len());

    let result = fs::read_dir(read_dir.trim()).expect("读取目录失败");
    let input_file_path = format!("{}/input.txt", read_dir.trim());
    let mut input_file = fs::File::create(&input_file_path).expect("创建输入文件失败");
    let mut input_str = String::new();

    for entry in result {
        let entry = entry.unwrap();
        let file_path = entry.path();
        let file_path_str = file_path.as_path();
        let file_name = file_path_str.file_name().unwrap();
        let file_name_str = file_name.to_str().unwrap();
        let cs = String::from(file_name_str);

        input_str.push_str("file '");
        input_str.push_str(cs.as_str());
        input_str.push_str("'\n");
    }

    input_file.write(input_str.as_bytes()).expect("写入错误");

    Command::new("ffmpeg")
        .current_dir(read_dir.trim())
        .arg("-f")
        .arg("concat")
        .arg("-i")
        .arg("./input.txt")
        .arg("-c")
        .arg("copy")
        .arg("output.mp4")
        .output()
        .expect("合并命令执行失败");

    fs::remove_file(input_file_path).expect("删除输入临时文件失败");
}
