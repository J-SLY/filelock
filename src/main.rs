use std::process;

// use std::fs;
use clap::Parser;
#[derive(Parser,Debug)]
#[command(author,version,about,long_about=None)]
struct Args{

    #[arg(short,long)]
    filename:String,

    #[arg(short,long)]
    offset:i32
}
fn main() {
    let args = Args::parse();
    
}
