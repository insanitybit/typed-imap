

extern crate openssl;

use imaperror::IMAPError;
use std::time::Duration;
use std::net::TcpStream;
use self::openssl::ssl::{SslContext, SslStream, SslMethod, Ssl};
use std::io::{Write, Read};
use std::str;

#[derive(Debug)]
pub enum IMAPConnection {
    Basic(TcpStream),
    Ssl(SslStream<TcpStream>),
    Disconnected,
}


impl IMAPConnection {
    pub fn new() -> IMAPConnection {
        IMAPConnection::Disconnected
    }

    pub fn new_notls(host: &str, port: u32) -> Result<IMAPConnection, IMAPError> {
        let host = host;
        let server = format!("{}:{}", host, port);

        let stream = try!(TcpStream::connect(&*server));
        try!(stream.set_read_timeout(Some(Duration::from_secs(15))));
        try!(stream.set_write_timeout(Some(Duration::from_secs(15))));

        Ok(IMAPConnection::Basic(stream))
    }

    pub fn new_tls(host: &str, port: u32) -> Result<IMAPConnection, IMAPError> {
        let host = host;
        let server = format!("{}:{}", host, port);

        let stream = {
            let stream = try!(TcpStream::connect(&*server));
            let _ = stream.set_read_timeout(Some(Duration::from_secs(15)));
            let _ = stream.set_write_timeout(Some(Duration::from_secs(15)));

            let sslcontext = try!(SslContext::new(SslMethod::Sslv23));
            let ssl = try!(Ssl::new(&sslcontext));
            let stream = try!(SslStream::connect(ssl, stream));

            stream
        };

        Ok(IMAPConnection::Ssl(stream))
    }

    pub fn command(&mut self, cmd: &str) -> Result<Vec<u8>, IMAPError> {
        println!("{:?}", cmd);
        match *self {
            IMAPConnection::Basic(ref mut stream) => {
                let _ = stream.write(cmd.as_bytes());
                let mut buf = Vec::new();
                let mut counter = 5;

                while buf.is_empty() && counter > 0 {
                    counter -= 1;
                    let _ = stream.read_to_end(&mut buf);
                    println!("{}", str::from_utf8(&buf).unwrap());
                }
                Ok(buf)
            }
            IMAPConnection::Ssl(ref mut stream) => {
                let _ = stream.write(cmd.as_bytes());
                let mut buf = Vec::new();
                let mut counter = 5;

                while buf.is_empty() && counter > 0 {
                    counter -= 1;
                    let _ = stream.read_to_end(&mut buf);
                    println!("{}", str::from_utf8(&buf).unwrap());
                }
                Ok(buf)
            }
            IMAPConnection::Disconnected => {
                Err(IMAPError::LoginError("Not connected to server.".to_owned()))
            }
        }
    }
}
