use std::{fs, process};
use clap::Parser;

#[derive(Parser,Debug)]
#[command(author,version,about,long_about=None)]
struct Args{

    #[arg(short,long)]
    filename:String,

    #[arg(short,long)]
    offset:usize

}
fn read_file(filename:&String)->Vec<u8>{
    match fs::read(filename){
        Ok(data) => {
            if data.len() != 0 {
                data
            }else{
                eprintln!("文件不能为空");
                process::exit(1);
            }
        },
        Err(e) => {
            eprintln!("读取文件失败，{}",e);
            process::exit(1);
        }
    }
}
fn check_offset(offset:usize,data:&Vec<u8>)->usize{
    if offset >= data.len() {
        offset % data.len()
    }else{
        offset
    }
}
fn file_lock(data:&mut Vec<u8>,offset:u8){
    for byte in data.iter_mut() {
        *byte ^= offset;
    }
}
fn write_file(data:&Vec<u8>,filename:&String){
    if let Err(e) = fs::write(filename,data){
        eprintln!("文件写入失败，{}",e);
        process::exit(1);
    }
}
fn main() {
    let mut args = Args::parse();
    let data = read_file(&args.filename);
    args.offset = check_offset(args.offset, &data);
    let key = (args.offset % 256) as u8;
    let mut data = data;
    file_lock(&mut data,key);
    write_file(&data, &args.filename);
}
