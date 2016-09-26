extern crate log;
extern crate hyper;

use std::io;
use std::io::{Take, Read};
use std::sync::{mpsc, Mutex, Arc};
use std::time::Duration;

use hyper::client::{Client, Request, Response, DefaultTransport as HttpStream};
use hyper::header::Connection;
use hyper::{Decoder, Encoder, Next};

#[derive(Debug)]
struct Dump {
    data: Vec<u8>,
    sender: mpsc::Sender<Vec<u8>>
}

impl Dump {
    #[inline]
    pub fn data(&self) -> Vec<u8> {
        return self.data.clone();
    }
}

impl Drop for Dump {
    #[inline]
    fn drop(&mut self) {
        let _ = self.sender.send(self.data.to_owned());
    }
}

fn read() -> Next {
    Next::read().timeout(Duration::from_secs(10))
}

impl hyper::client::Handler<HttpStream> for Dump {
    #[inline]
    fn on_request(&mut self, req: &mut Request) -> Next {
        req.headers_mut().set(Connection::close());
        read()
    }

    #[inline]
    fn on_request_writable(&mut self, _encoder: &mut Encoder<HttpStream>) -> Next {
        read()
    }

    #[inline]
    fn on_response(&mut self, res: Response) -> Next {
        read()
    }

    #[inline]
    fn on_response_readable(&mut self, decoder: &mut Decoder<HttpStream>) -> Next {
        match io::copy(decoder, &mut self.data) {
            Ok(0) => Next::end(),
            Ok(_) => read(),
            Err(e) => match e.kind() {
                io::ErrorKind::WouldBlock => Next::read(),
                _ => Next::end()
            }
        }
    }

    #[inline]
    fn on_error(&mut self, err: hyper::Error) -> Next {
        Next::remove()
    }
}

pub struct HTTP;

impl HTTP {
    #[inline]
    pub fn new() -> HTTP {
        return HTTP {};
    }

    #[inline]
    pub fn get(&mut self, url: &str) -> Vec<u8> {
        let (tx, rx) = mpsc::channel();
        let client = Client::new().expect("Failed to create a Client");

        let mut dump = Dump {
            data: Vec::new(),
            sender: tx
        };

        client.request(url.parse().unwrap(), dump).unwrap();

        let data = rx.recv().unwrap();

        client.close();

        return data;
    }
}