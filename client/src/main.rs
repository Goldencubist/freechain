use std::{
    net::TcpStream, io::{
        Write,
        Read,
        Result,
        ErrorKind::{
            TimedOut,
            ConnectionRefused
        }
    },
    thread::sleep,
    time::Duration
};

fn main() -> Result<()> {
    let target_ip = "192.168.1.46:8888";
    let mut stream = loop {
        match TcpStream::connect(target_ip) {
            Ok(s) => { break s }
            Err(e) => match e.kind() {
                ConnectionRefused => println!("Connection refused: trying again later"),
                TimedOut => println!("Timeout: trying again later"),
                _ => panic!("UNKNOWN ERROR {:?}", e)
            },
        }
        sleep(Duration::from_secs(2));
    };
    let mut packet: [u8; 2] = [1, 2];
    let mut running = true;
    println!("Running");
    while running {
        println!(
            "Sending packet [{}, {}] to target ip {}",
            packet[0],
            packet[1],
            target_ip
        );
        stream.write_all(&packet).unwrap();
        println!("Packet sent sucessfully\n");
        stream.read_exact(&mut packet)?;
        println!(
            "Packet [{}, {}] received from ip {}",
            packet[0],
            packet[1],
            target_ip
        );
        if packet[0] == 255 && packet[1] == 255 {
            println!("Packet exchange finished");
            running = false
        }
    }
    println!("Shutting down");
    Ok(())
}