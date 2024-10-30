use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream}
};

use http_server::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7777").unwrap();
    let pool = ThreadPool::new(4).unwrap();

    for stream in listener.incoming().take(4) {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buffer_reader = BufReader::new(&mut stream);

    let http_request: Vec<_> = buffer_reader
        .lines()
        .map(|line| line.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let request_line = &http_request[0];

    println!("Request: {:#?}", request_line);

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "index.html"),
        "GET /sleep HTTP/1.1" => {
            std::thread::sleep(std::time::Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "index.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!(
        "{status_line}\r\n\
         Content-Length: {length}\r\n\r\n\
         {contents}"
    );

    stream.write_all(response.as_bytes()).unwrap();
}
