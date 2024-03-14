use std::{ fs, io::{BufRead, BufReader, Write}, net::{TcpListener, TcpStream}, thread, time::Duration};

use web_server::ThreadPool;

fn main() {
    
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();// creating a localserver and bind with it

    let pool= ThreadPool::new(4);// make a threadpool with custom size

    for stream in listener.incoming().take(2){// take()-- how much thread we can use, after using those thread, the programme stoppps

        let strm= stream.unwrap();
        println!("Connection established ");   
        
        pool.execute(||{
            handle_connection4(strm);
        });
    };


}



fn handle_connection4(mut stream: TcpStream){
    let buf_reader= BufReader::new(&mut stream);// TODO: why we need to store this to buffer
    let request_line= buf_reader.lines().next().unwrap().unwrap();
    
    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "sleep.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };


    let contents= fs::read_to_string(filename).unwrap() ;
    let length= contents.len();

    let response= format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    
    stream.write_all(response.as_bytes()).unwrap()  ;
}




#[allow(dead_code)]
    fn handle_connection3(mut stream: TcpStream){
        let buf_reader= BufReader::new(&mut stream);
        let request_line= buf_reader.lines().next().unwrap().unwrap();
    
        let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
            ("HTTP/1.1 200 OK", "hello.html")
        } else {
            ("HTTP/1.1 404 NOT FOUND", "404.html")
        };
    
        let contents= fs::read_to_string(filename).unwrap() ;
        let length= contents.len();
    
        let response= format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
        
        thread::sleep(Duration::from_secs(5));
        stream.write_all(response.as_bytes()).unwrap()  ;
    }


    #[allow(dead_code)]
fn handle_connection1(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
   
    let _http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let status_line = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string("hello.html").unwrap();
    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}


#[allow(dead_code)]

fn handle_connection2(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    if request_line == "GET / HTTP/1.1" {
        let status_line = "HTTP/1.1 200 OK";
        let contents = fs::read_to_string("hello.html").unwrap();
        let length = contents.len();

        let response = format!(
            "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
        );

        stream.write_all(response.as_bytes()).unwrap();
    } else {
        let status_line = "HTTP/1.1 404 NOT FOUND";
        let contents = fs::read_to_string("404.html").unwrap();
        let length = contents.len();

        let response = format!(
            "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
        );

        stream.write_all(response.as_bytes()).unwrap();
        let status_line = "HTTP/1.1 404 NOT FOUND";
        let contents = fs::read_to_string("404.html").unwrap();
        let length = contents.len();

        let response = format!(
            "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
        );

        stream.write_all(response.as_bytes()).unwrap();
    
    }

    println!("Rquest:  {}",request_line);   
}




