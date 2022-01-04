mod threadpool;
use std::io::prelude::*;
fn main() {
    setup_web_server()
}

fn setup_web_server() {
    let tcp_listner = std::net::TcpListener::bind("0.0.0.0:6969").unwrap();
    let pool = threadpool::Threadpool::new(50);

    for stream in tcp_listner.incoming() {
        pool.execute(|| handle_connects(stream.unwrap()))
    }
}

fn handle_connects(mut stream: std::net::TcpStream) {
    let mut requestbuffer = [0; 1024];
    stream.read(&mut requestbuffer).unwrap();
    let request = String::from_utf8_lossy(&requestbuffer);
    let index = request.find("HTTP").unwrap();
    let reqpath = request[4..index - 1].to_string();
    match reqpath.as_str() {
        "/" => {
            println!("User requested Main Page!");
            let mainpage = std::fs::read_to_string("main.hmtl").unwrap();
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
                mainpage.len(),
                mainpage
            );
            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        }
        "/shutdown" => {
            println!("User requested shutdown!");
            let shutdownpage = std::fs::read_to_string("shutdown.hmtl").unwrap();
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
                shutdownpage.len(),
                shutdownpage
            );
            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
            std::process::exit(0)
        }
        _ => {
            println!("User requested unknown path! Serving 404!");
            let unknownpage = std::fs::read_to_string("404.hmtl").unwrap();
            let response = format!(
                "HTTP/1.1 404 NOT FOUND\r\nContent-Length: {}\r\n\r\n{}",
                unknownpage.len(),
                unknownpage
            );
            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        }
    }
}
