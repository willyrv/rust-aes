use aes::Aes128;
use aes::cipher::{
    BlockCipher, BlockEncrypt, BlockDecrypt, KeyInit,
    generic_array::GenericArray,
};
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::str;

fn main() -> io::Result<()>{
    let mut f = File::open("./text2encrypt.txt")?;
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer)?;
    let text = str::from_utf8(&buffer).unwrap();

    println!("Hello, world!");
    println!("{:?}", buffer);
    //println!("{:?}", block)
    println!("------------------");
    println!("{}", text);
    Ok(())
}
