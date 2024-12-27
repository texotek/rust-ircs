use std::{io::{BufRead, BufReader, BufWriter}, net::TcpListener};

fn main() {
    let socket = TcpListener::bind("localhost:6667").unwrap();

    let (stream, _) = socket.accept().unwrap();

    let reader = BufReader::new(&stream);
    reader.lines().for_each(|l| println!("{}", l.unwrap()));

}
