use std::collections::VecDeque;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::time::Duration;

const DEFAULT_TIMEOUT: Option<Duration> = Some(Duration::from_millis(1000));

fn main() -> std::io::Result<()> {
    let queue: Arc<Mutex<VecDeque<String>>> = Arc::new(Mutex::new(VecDeque::new()));
    let listener = TcpListener::bind("127.0.0.1:7878")?;

    for stream in listener.incoming() {
        let cloned_queue = queue.clone();
        std::thread::spawn(move || match stream {
            Ok(stream) => {
                if let Err(e) = handle_client(stream, &cloned_queue) {
                    eprintln!("Error handling client: {:?}", e);
                }
            }
            Err(e) => eprintln!("Error connecting: {:?}", e),
        });
    }

    Ok(())
}

fn handle_client(mut stream: TcpStream, queue: &Mutex<VecDeque<String>>) -> std::io::Result<()> {
    stream.set_read_timeout(DEFAULT_TIMEOUT)?;
    stream.set_write_timeout(DEFAULT_TIMEOUT)?;

    let mut buffer = String::new();
    stream.read_to_string(&mut buffer)?;

    match simple_db::parse(&buffer) {
        Ok(command) => match command {
            simple_db::Command::Publish(payload) => {
                queue.lock().unwrap().push_back(payload);
                writeln!(stream, "Added to queue: {buffer:?}")?;
            }
            simple_db::Command::Retrieve => {
                writeln!(
                    stream,
                    "Retrieved from queue: {:?}",
                    queue.lock().unwrap().pop_front()
                )?;
            }
        },
        Err(e) => writeln!(stream, "Error: {:?}", e)?,
    }

    Ok(())
}
