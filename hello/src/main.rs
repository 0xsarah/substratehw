use std::fs; //for file reading later
use std::io::prelude::*; //looking at input&output
use std::net::TcpListener; //these two std are for tcp
use std::net::TcpStream;


fn main() {
    //listening for a connection on port 7878 and calls handle connection to return result
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    //putting the stream into a buffer, if response starts with get, return hello.html otherwise return 404.html
    let mut buffer = [0; 1024]; //set buffer size to 1024 for convenience, can increase in the future
    stream.read(&mut buffer).unwrap(); //read/unwrap buffer

    let get = b"GET / HTTP/1.1\r\n"; //catch http/1.1 requests, if request is higher than http/1.1 there will be an error

    if buffer.starts_with(get) {
        //match buffer with get request
        let contents = fs::read_to_string("hello.html").unwrap();
        //convert contents into html friendly  format
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            contents.len(),
            contents
        );
        //write response back to stream
        stream.write(response.as_bytes()).unwrap();
        //clear the stream
        stream.flush().unwrap();
    
    } else {
        let status_line = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
        let contents = fs::read_to_string("404.html").unwrap();
        //convert contents to html friendly format

        let response = format!("{}{}", status_line, contents);
        //write response back to stream
        stream.write(response.as_bytes()).unwrap();
        //clear the stream
        stream.flush().unwrap();
    }
}
