use std::{process::Stdio};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream},
    process::Command,
};

// Model Server describe principal characteristic about 
// -------------------- SERVER -------------------------

// host describe in IPv4 about direct connection
// port is the 22 because this is standar or default ssh

// listener save or added socket with conection with this
// channels save sockets with connection to clients

pub struct Server {
    pub host: &'static str,
    pub port: &'static str,
    pub listener: Option<TcpListener>,
    pub channels: Option<Vec<TcpStream>>,
}

impl Server {
    // Init principal funtion or method with intention 
    // started server and save socket on model Server
    // and listener in this port incomming connections
    pub async fn init(self) -> Result<(), Box<dyn std::error::Error>> {
        let address = format!("{}:{}", self.host, self.port);
        let listener = TcpListener::bind(address)
            .await
            .expect("Error run server on port {address}");

        println!("INFO Server listener on port {}", self.port);

        loop {
            let (socket, addr) = listener.accept().await?;
            println!("CONN from {}", addr);

            // start loop with intention listener incomming connection
            tokio::spawn(async move {
                if let Err(e) = handle_connection(socket).await {
                    eprintln!("ERROR: connection with {} failed: {}", addr, e);
                }
            });
        }
    }
}

async fn handle_connection(socket: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    // socket listener writer in console of client
    // this client write commands in console this impact
    // about our system and wait an answer
    let (reader, mut writer) = socket.into_split();
    let mut reader = BufReader::new(reader);
    let mut line = String::new();

    loop {
        line.clear();
        let bytes_read = reader.read_line(&mut line).await?;

        if bytes_read == 0 {
            println!("INFO: Client disconnected");
            break;
        }
        
        let parts = match shlex::split(&line) {
            Some(v) if !v.is_empty() => v,
            _ => {
                eprintln!("No se pudo parsear la línea o está vacía.");
                continue;
            }
        };

        let program = &parts[0];
        let args = &parts[1..];

        // method writed for client if != NULL
        // run in our system and generate response 
        // for our client and sending than aswr
        let mut child = Command::new(program)
            .args(args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("No se pudo ejecutar el comando");

        if let Some(stdout) = child.stdout.take() {
            let mut lines = BufReader::new(stdout).lines();

            while let Some(line) = lines.next_line().await? {
                writer
                    .write_all(format!("{}\n", line).as_bytes())
                    .await?;
            }
        }
    }

    Ok(())
}
