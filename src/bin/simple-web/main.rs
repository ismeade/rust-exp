use std::{io::{BufReader, prelude::*}, net::{TcpListener, TcpStream}, thread, time};
use std::fmt::{Formatter, Pointer};
use std::time::Duration;
use rust_exp::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        });

    }

}

/// `handle_connection` handle处理
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
fn handle_connection(mut stream: TcpStream) {
    println!("handle_start");
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    println!("Request: {:#?}", request_line);

    let (status_line, contents) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello world"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "sleep")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "not found"),
    };

    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();

    println!("handle_end");
}
