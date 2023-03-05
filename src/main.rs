use std::net::UdpSocket;
use std::thread;

const MAX_MESSAGE_SIZE: usize = 1024;

fn start_client(socket: &UdpSocket) {
    let mut buf = String::new();

    loop {
        match std::io::stdin().read_line(&mut buf) {
            Ok(_) => {}
            Err(error) => {
                eprintln!("Error reading input: {}", error);
                continue;
            }
        };

        let _ = socket.send_to(buf.as_bytes(), "127.0.0.1:34254");

        if buf.trim_end() == "/quit" {
            break;
        }

        buf = String::new();
    }
}

fn main() {
    let client_address = "127.0.0.1:0";

    let socket = UdpSocket::bind(client_address).expect("Failed to bind server socket");
    let s2 = socket.try_clone().unwrap();
    let _ = thread::spawn(move || start_client(&s2));

    let mut buf = [0u8; MAX_MESSAGE_SIZE];

    println!("Welcome to the chat!\nEnter your name to connect to the chat");

    loop {
        match socket.recv_from(&mut buf) {
            Ok((size, _origin)) => {
                let message = String::from_utf8_lossy(&buf[..size]);
                println!("{}", message.trim_end());

                buf.fill(0);
            }
            Err(e) => {
                eprintln!("Error receiving message: {}", e);
                break;
            }
        }
    }
}
