use std::{io,net::TcpListener,net::TcpStream};
use std::sync::atomic::{AtomicBool, Ordering};

pub struct Server {
    shouldStop: AtomicBool
}


impl Server {
    pub fn new() -> Server {
        Server{
            shouldStop: AtomicBool::new(false)
        }        
    }

    pub fn start(&self,port: usize) -> io::Result<()> {
        let listener = TcpListener::bind("0.0.0.0:3000")?;

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    handle_connection(stream);
                }
                Err(_) => {
                    
                }
            }
        }
        Ok(())
    }

    fn stop(self) {
        self.shouldStop.swap(true,Ordering::Relaxed);
    }

}

fn handle_connection(stream: TcpStream) {
    println!("connected");
}