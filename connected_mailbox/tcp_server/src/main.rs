use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::time::Duration;
use std::collections::VecDeque;

const DEFAULT_TIMEOUT: Option<Duration> = Some(Duration::from_millis(1000));

fn main() -> std::io::Result<()> {
    let mut queue: VecDeque<String> = VecDeque::new();
    let listener = TcpListener::bind("127.0.0.1:7878")?;

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                if let Err(e) = handle_client(stream, &mut queue) {
                    eprintln!("Error handling client: {:?}", e);
                }
            }
            Err(e) => eprintln!("Error connecting: {:?}", e),
        }
    }

    Ok(())
}

fn handle_client(mut stream: TcpStream, queue: &mut VecDeque<String>) -> std::io::Result<()> {
    stream.set_read_timeout(DEFAULT_TIMEOUT)?;
    stream.set_write_timeout(DEFAULT_TIMEOUT)?;

    let mut buffer = String::new();
    stream.read_to_string(&mut buffer)?;
    if let Ok(command) = simple_db::parse(&buffer) {
        match command {
            simple_db::Command::Publish(payload) => {
                queue.push_back(payload);
                writeln!(stream, "Added to queue: {buffer:?}")?;
            },
            simple_db::Command::Retrieve => {
                writeln!(stream, "Retrieved from queue: {:?}", queue.pop_front())?;
            }
        }
    } else {

    }
    writeln!(stream, "Thank you for {buffer:?}!")?;
    Ok(())
}
