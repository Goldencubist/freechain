use std::{
    io::{
        Read,
        Write,
        Result
    },
    net::TcpListener
};

fn main() -> Result<()> {
    let listener = TcpListener::bind("0.0.0.0:8888")?;
    let mut on_same_connection: bool;
    println!("TCP server listening at port 8888");
    for stream in listener.incoming() {
        on_same_connection = true;
        let mut stream = stream?;
        let worker_node_ip = stream.peer_addr()?;
        println!("Worker node connected from ip {}", worker_node_ip);
        let mut buf = [0_u8; 2];
        while on_same_connection {
            if let Ok(()) = stream.read_exact(&mut buf){
                println!(
                    "Received data from worker node '{}':\nNumber 1: {}\nNumber 2: {}\n",
                    worker_node_ip,
                    buf[0],
                    buf[1]
                );
                if buf[0] < 255 { buf[0] += 1 }
                if buf[1] < 255 { buf[1] += 1 }
                println!(
                    "Updated data to be returned to node '{}':\nNumber 1: {}\nNumber 2: {}\n",
                    worker_node_ip,
                    buf[0],
                    buf[1]
                );
                stream.write_all(&buf)?;
                println!("Data sent sucessfully");
                if buf == [255, 255] { on_same_connection = false }
            }
        }
    }
    Ok(())
}
