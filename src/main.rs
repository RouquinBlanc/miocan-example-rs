extern crate mio;
extern crate miocan;
extern crate socketcan;

use std::io;
use mio::*;
use miocan::MIOCANSocket;
use socketcan::CANSocket;

fn main() {
    let socket = CANSocket::open("vcan0").expect("creating CAN socket");
    socket.set_nonblocking(true).expect("set socket non-blocking");
    let mio_socket = MIOCANSocket::from(socket);

    let poll = Poll::new().expect("creating poll");
    poll.register(&mio_socket, Token(0), Ready::readable(), PollOpt::edge()).unwrap();

    let mut events = Events::with_capacity(1024);

    loop {
        poll.poll(&mut events, None).unwrap();

        for event in events.iter() {
            match event.token() {
                _ => {
                    loop {
                        // A frame should be ready
                        match mio_socket.read_frame() {
                            Ok(frame) => println!("{:?}", frame),
                            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => break,
                            Err(e) => panic!("err={}", e)
                        }
                    }
                }
            }
        }
    }
}
