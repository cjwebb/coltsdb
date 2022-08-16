use std::io::prelude::*;
use std::io::BufReader;
use std::net::TcpListener;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::thread;

extern crate serde_json;
use serde_json::Error;

mod simple_db;
use simple_db::SimpleDB;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap(); // todo - config-driven binding

    let db = Arc::new(Mutex::new(SimpleDB::new()));

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let db = Arc::clone(&db);
        thread::spawn(move || {
            handle_connection(stream, db);
        });
    }
}

// Take stream, parse, and pass things off to database class.
fn handle_connection(mut stream: TcpStream, db: Arc<Mutex<SimpleDB>>) {
    let mut reader = BufReader::new(&stream);
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();

    let request: Result<simple_db::Request, Error> = serde_json::from_str(line.as_str());

    match request {
        Ok(req) => match req {
            simple_db::Request::Insert { key, value } => {
                let mut d = db.lock().unwrap();
                d.put(key, value);
                stream.write(b"insert done").unwrap();
            }
            simple_db::Request::Query { key } => {
                let db = db.lock().unwrap();
                let value = db.get(key);
                match value {
                    Some(v) => stream.write(v.as_bytes()).unwrap(),
                    None => stream.write(b"Not Found").unwrap(),
                };
            }
        },
        Err(_) => {
            stream.write(b"Error").unwrap();
        }
    }

    stream.flush().unwrap();
}
