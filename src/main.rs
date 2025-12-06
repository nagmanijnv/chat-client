use std::error::Error;
use std::io::{self, Write};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("usage: cargo run <port>");
        std::process::exit(1);
    }

    let port: u16 = args[1]
        .parse()
        .map_err(|_| "Failed to parse port, port must be a number".to_string())?;
    let addr = format!("127.0.0.1:{port}");
    println!("Connecting to {}...", addr);

    // Connect to the server
    let stream = TcpStream::connect(addr).await?;
    println!("Connected!");

    let (reader, mut writer) = stream.into_split();
    let mut socket_reader = BufReader::new(reader);

    // Channel to send/receive message to/from stdout/stdin
    // stdin -> receiver -> writer task
    // reader task -> stdout 
    let (tx, mut rx) = mpsc::channel::<String>(100);

    // Spawn a task to read messages from server and print them on to stdout
    tokio::spawn(async move {
        let mut buf = String::new();
        loop {
            buf.clear();

            let n = match socket_reader.read_line(&mut buf).await {
                Ok(n) => n,
                Err(err) => {
                    eprintln!("Error reading socket: {}", err);
                    break;
                }
            };
            if n == 0 {
                println!("Disconnected from server.");
                std::process::exit(0);
            }
            // writing to stdout, later flushing stdout
            print!("{}", buf);
        }
    });

    // Spawn a task to read from stdin and send to writer
    tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            // write to the writer stream
            if writer.write_all(msg.as_bytes()).await.is_err() {
                println!("Server closed connection.");
                std::process::exit(0);
            }
        }
    });

    // flush stdout
    // read from stdin and send through channel
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        // send to channel
        tx.send(input).await?;
    }
}
