use std::{cmp::Ordering, fs, process};
use clap::{Parser, Subcommand};


/// Gat Args 
/// 
/// `filename:String`,`offset:usize`,`is_mod:bool`
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

        #[arg(short)]
        m:bool,
    }
}
/// # Example
/// using **`let data:Vex<u8> = read_file(&file_name);`**
/// to get file data, return a Vec\<u8\>.
/// 
fn read_file(filename:&String)->Vec<u8>{
    match fs::read(filename){
        Ok(data) => {
            match data.len().cmp(&0usize){
                Ordering::Equal|Ordering::Less => {
                    eprintln!("文件不能为空");
                    process::exit(1);
                },
                Ordering::Greater => data
            }
        },
        Err(e) => {
            eprintln!("读取文件失败，{e}");
            process::exit(1);
        }
    }
}
/// # Example
/// using **`let key = check_offset(offset,&data,is_mod);`** to get right offset.
/// 
/// if `is_mod` is false and offset is greater than length of data, this function will throw an error and exit.
///
/// if `is_mod` is true, this function will return right offset.
fn check_offset(offset:usize,data:&Vec<u8>,is_mod:bool)->usize{
    match is_mod{
        true => offset%data.len(),
        false => {
            match offset.cmp(&data.len()){
                Ordering::Greater|Ordering::Equal => {
                    eprintln!("偏移量（{}）大于文件大小（{}）",offset,data.len());
                    process::exit(1);
                },
                Ordering::Less => offset
            }
        }
    }
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
fn main() {
    let args = Args::parse();
    match args.command {
        Commands::Lock { filename, offset, m } =>{
            let data = read_file(&filename);

        let offset = check_offset(offset, &data,m);
        
        let key = (offset % 256) as u8;
        let mut data = data;

        file_lock(&mut data,key);
        write_file(&data, &filename);

        println!("文件{}已关于密钥{}异或",filename,key);
        
        }
    }
    
}
