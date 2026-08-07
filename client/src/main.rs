use std::{
    net::TcpStream,
    io::{
        self,
        Write,
        Result,
        ErrorKind::{
            TimedOut,
            ConnectionRefused
        }
    },
    thread::sleep,
    time::Duration
};
use serde_json;
mod core;

fn main() -> Result<()> {
    let target_ip = "192.168.1.46:8888";
    let mut stream = loop {
        match TcpStream::connect(target_ip) {
            Ok(s) => { break s }
            Err(e) => match e.kind() {
                ConnectionRefused => eprintln!("Conexão refusada: tentando novamente mais tarde"),
                TimedOut => eprintln!("Timeout: tentando novamente mais tarde"),
                _ => panic!("ERRO DESCONHECIDO {:?}", e)
            },
        }
        sleep(Duration::from_secs(2));
    };
    loop {
        let mut text = String::new();
        print!("Digite uma mensagem para enviar para o servidor: ");
        io::stdout().flush()?; io::stdin().read_line(&mut text)?;
        let mut author = String::new();
        print!("Qual é o nome do autor? ");
        io::stdout().flush()?; io::stdin().read_line(&mut author)?;
        let mut receiver = String::new();
        print!("Para quem é a mensagem? ");
        io::stdout().flush()?; io::stdin().read_line(&mut receiver)?;
        let data = core::structs::Message{
            origin: String::from(author.trim()),
            destination: String::from(receiver.trim()),
            text: String::from(text.trim())
        };
        let data_parsed = serde_json::to_string(&data)?;
        let data_bytes = data_parsed.as_bytes();
        let data_len = data_parsed.len() as u32;
        let data_len = data_len.to_be_bytes();
        let protocol = [1_u8, 2];
        println!("Rodando");
        println!(
            "Enviando mensagem para o ip {}",
            target_ip
        );
        stream.write_all(&protocol)?;
        stream.write_all(&data_len)?;
        stream.write_all(data_bytes)?;
        println!("Texto enviado com sucesso");
    }
}