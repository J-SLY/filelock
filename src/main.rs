use std::{cmp::Ordering, fs, process};
use clap::{Parser, Subcommand};

#[derive(Parser,Debug)]
#[command(author,version,about,long_about=None)]
struct Args{
    #[command(subcommand)]
    command:Commands,
    
}

#[derive(Subcommand, Debug)]
enum Commands{
    Lock{
        #[arg(short,long)]
        filename:String,

        #[arg(short,long)]
        offset:usize,
    },
    Unlock{
        #[arg(short,long)]
        filename:String,
    }
}
 
fn read_file(filename:&String)->Vec<u8>{
    match fs::read(filename){
        Ok(data) => {
            match data.len().cmp(&0usize){
                Ordering::Equal => {
                    eprintln!("文件不能为空");
                    process::exit(1);
                },
                _ => data
            }
        },
        Err(e) => {
            eprintln!("读取文件失败，{e}");
            process::exit(1);
        }
    }
}

fn check_offset(offset:usize,data:&Vec<u8>)->usize{
    let len = data.len();
    if len == 0 { return 0; }
    let result = offset % len;
    if result == 0 && offset != 0 { len } else { result }
}

fn file_lock(data:&mut Vec<u8>,offset:u8){
    for byte in data.iter_mut() {
        *byte ^= offset;
    }
}

fn write_file(data:&Vec<u8>,filename:&String){
    match fs::write(filename,data){
        Ok(_) => (),
        Err(e) => {
            eprintln!("文件写入失败,{e}");
            process::exit(1);
        }
    }
}

fn add_key(data:&mut Vec<u8>,key:u8){
    data.push(key);
}

fn main() {
    let args = Args::parse();
    match args.command {
        Commands::Lock { filename, offset } =>{
            let data = read_file(&filename);

            let offset = check_offset(offset, &data);
            
            let key = (offset % 256) as u8;
            let mut data = data;

            file_lock(&mut data,key);
            add_key(&mut data, key);
            write_file(&data, &filename);

            println!("文件{}已用密钥{}加密",filename,key);
        
        },
        Commands::Unlock{ filename }=>{
            let mut data = read_file(&filename);
            if data.is_empty() {
                eprintln!("文件不能为空或没有密钥");
                process::exit(1);
            }
            // 末尾字节作为密钥，读取并移除
            let key = data.pop().unwrap();
            // XOR 对称，直接使用相同函数解密
            file_lock(&mut data, key);
            write_file(&data, &filename);
            println!("文件{}已用密钥{}解密并移除密钥", filename, key);
        }
    }
}