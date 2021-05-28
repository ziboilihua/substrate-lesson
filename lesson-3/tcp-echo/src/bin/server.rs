use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::str::from_utf8;
use std::thread;

// echo data which are recieved from client back to client
fn echo(mut stream: TcpStream) {
    const MAX_BUFFER_LEN: usize = 1024;
    let mut data = [0 as u8; MAX_BUFFER_LEN];

    // read data from client
    while match stream.read(&mut data) {
        // if read correctly, write back data to client
        Ok(size) => {
            let recv_data = from_utf8(&data).unwrap();
            println!("server recv: {}", recv_data);
            stream.write(&data[0..size]).unwrap();
            true // size == MAX_BUFFER_LEN
        }

        // if error occurs, shutdown tcp stream
        Err(_) => {
            println!(
                "error occured: peer addr is {:?}",
                stream.peer_addr().unwrap()
            );
            // shutdown both tcp stream
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

fn main() {
    // bind tcp server to port 3333
    let listener = TcpListener::bind("127.0.0.1:3333").unwrap();

    println!("Tcp Server is listening on port 3333");

    // process incoming tcp client
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());

                // connect success and spawn a new thread to echo recieve data
                thread::spawn(move || echo(stream));
            }
            Err(e) => {
                // connect failed and print error message
                println!("Error: {}", e);
            }
        }
    }
}
