use std::{
    io::{
        Read,
        Result
    },
    net::TcpListener,
    thread
};
use serde_json;

use crate::core::structs::Message;
mod core;

fn main() -> Result<()> {
    let listener = TcpListener::bind("0.0.0.0:8888")?;
    println!("Servidor TCP escutando na porta 8888");
    for stream in listener.incoming() {
        thread::spawn(move || -> Result<()> {
            let mut stream = stream?;
            let client_ip = stream.peer_addr()?;
            println!("Cliente conectou do ip {}", client_ip);
            loop {
                let mut header = [0_u8; 6];
                stream.read_exact(&mut header)?;
                match (header[0], header[1]) {
                    (1, 2) => {
                        let data_len = u32::from_be_bytes([
                            header[2],
                            header[3],
                            header[4],
                            header[5]
                        ]) as usize;
                        let mut data = vec![0_u8; data_len];
                        if let Err(e) = stream.read_exact(&mut data) {
                            println!("Cliente {} desconectou. Erro: {:?}", client_ip, e);
                            break;
                        }
                        let data: core::structs::Message = match serde_json::from_slice(&data){
                            Ok(msg) => msg,
                            Err(e) => {
                                eprintln!("Erro ao desserializar dados do cliente {}: {:?}", client_ip, e); continue;
                            }
                        };
                        println!(
                            "Dados recebidos do cliente '{}':",
                            client_ip
                        );
                        println!(
                            "Autor: {}\nPara: {}\nMensagem: {}",
                            data.origin,
                            data.destination,
                            data.text
                        )
                    }
                    _ => {eprintln!(
                    "Protocolo/Operação desconhecido(a). ID do protocolo: {}\n ID da operação: {}",
                    header[0], header[1]
                    )}
                }
            }
            Ok(())
        });
    }
    Ok(())
}
