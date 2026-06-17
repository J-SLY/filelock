use std::{env, fs, process};

struct Config{
    file_name:String,
    move_number:usize
}
fn get_config(args:Vec<String>) -> Config{
    if args.len() < 3{
        eprintln!("用法：程序名（{}） 文件名 偏移量",args[0]);
        process::exit(1)
    }
    let move_number: usize = match args[2]
        .trim()
        .parse(){
            Ok(num) => num,
            Err(_)  => {
                eprintln!("偏移量必须是整数");
                process::exit(1)
            }
        };
    Config{file_name:args[1].clone(),move_number:move_number}
}
fn collect_args() -> Config{
    get_config(env::args().collect())
}
fn read_file(filename:&String)->Vec<u8>{
    match fs::read(filename){
        Ok(bytes)=>bytes,
        Err(e)=>{
            eprintln!("读取文件失败，{e}");
            process::exit(1);
        }
    }
}
fn check_move_number(data:&Vec<u8>,move_number:usize) -> bool{
    if move_number >= data.len(){
        false
    }else{
        true
    }
}
fn main() {
    let config:Config = collect_args();
    let data = read_file(&config.file_name);
    if check_move_number(&data, config.move_number) == false{
        eprintln!("偏移量（{}）大于文件大小{}",config.move_number,data.len());
        process::exit(1)
    } 
    let key = (config.move_number % 256) as u8;
    let mut encrypted = data;
    for byte in encrypted.iter_mut().skip(config.move_number as usize) {
        *byte ^= key;
    }
    if let Err(e) = fs::write(config.file_name, &encrypted) {
        eprintln!("写入文件失败: {}", e);
        process::exit(1);
    }
    println!("加密/解密完成，使用偏移量 {}（密钥 0x{:02X}）", config.move_number, key);
}
