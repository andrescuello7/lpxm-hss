use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::thread;

fn main() -> io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:22")?;
    println!("Conectado al servidor.");

    let mut reader_stream = stream.try_clone()?;

    // Hilo: leer del servidor
    thread::spawn(move || {
        let mut buffer = [0; 1024];
        loop {
            match reader_stream.read(&mut buffer) {
                Ok(0) => {
                    println!("Servidor cerró la conexión.");
                    break;
                }
                Ok(n) => {
                    print!("{}\n", String::from_utf8_lossy(&buffer[..n]));
                }
                Err(err) => {
                    eprintln!("Error leyendo: {}", err);
                    break;
                }
            }
        }
    });

    // Hilo principal: leer stdin y mandar al server
    let stdin = io::stdin();
    let mut input = String::new();

    loop {
        input.clear();
        stdin.read_line(&mut input)?;
        stream.write_all(input.as_bytes())?;
    }
}
