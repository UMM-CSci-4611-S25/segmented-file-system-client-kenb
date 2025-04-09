// Below is a version of the `main` function and some error types. This assumes
// the existence of types like `FileManager`, `Packet`, and `PacketParseError`.
// You can use this code as a starting point for the exercise, or you can
// delete it and write your own code with the same function signature.

mod errors;
mod file_manager;
mod packet;
mod packet_group;

use std::{
    io::{self, Write},
    net::UdpSocket, time::{Duration, Instant},
};

use crate::{errors::ClientError, file_manager::FileManager, packet::Packet};

const LOCAL_ADDR: &str = "0.0.0.0:7077";
const REMOTE_ADDR: &str = "127.0.0.1:6014";

fn main() -> Result<(), ClientError> {
    let sock = UdpSocket::bind(LOCAL_ADDR)?;
    println!("Listening on {}", LOCAL_ADDR);

    sock.connect(REMOTE_ADDR)?;
    println!("Connected to {}", REMOTE_ADDR);
    println!("Waiting for packets...");

    let mut buf = [0; 1028];
    let _ = sock.send(&buf[..1028]);

    let mut file_manager = FileManager::default();

    while !file_manager.received_all_packets() {
        println!("Waiting to receive a packet...");
        let len = sock.recv(&mut buf)?;
        println!("Received {} bytes: {:?}", len, &buf[..len]);

        let packet: Packet = match buf[..len].try_into() {
            Ok(packet) => packet,
            Err(e) => {
                eprintln!("Error parsing packet: {:?}", e);
                continue;
            }
        };

        print!(".");
        println!("Received packet: {:?}", packet);
        io::stdout().flush()?;
        file_manager.process_packet(packet);
    }

    // let timeout = Duration::from_secs(5); // Set a timeout of 30 seconds
    // let start_time = Instant::now();
    // let max_iterations = 100; // Set a maximum iteration limit
    // let mut iteration_count = 0;

    // loop {
    //     if start_time.elapsed() > timeout {
    //         eprintln!("Error: Timeout reached while waiting for packets.");
    //         break;
    //     }

    //     if iteration_count >= max_iterations {
    //         eprintln!("Error: Maximum iteration limit reached.");
    //         break;
    //     }

    //     println!("Waiting to receive a packet...");
    //     let len = sock.recv(&mut buf)?;
    //     println!("Received {} bytes: {:?}", len, &buf[..len]);

    //     let packet: Packet = match buf[..len].try_into() {
    //         Ok(packet) => packet,
    //         Err(e) => {
    //             eprintln!("Error parsing packet: {:?}", e);
    //             continue;
    //         }
    //     };

    //     print!(".");
    //     println!("Received packet: {:?}", packet);
    //     io::stdout().flush()?;
    //     file_manager.process_packet(packet);

    //     if file_manager.received_all_packets() {
    //         break;
    //     }

    //     iteration_count += 1;
    // }

    if file_manager.received_all_packets() {
        println!("\nAll packets received. Writing files...");
        file_manager.write_all_files()?;
        println!("Files written successfully.");
    } else {
        eprintln!("Error: Failed to receive all packets.");
    }

    Ok(())
}
