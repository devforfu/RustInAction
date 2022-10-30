use std::io::prelude::*;
use std::net::TcpStream;

use reqwest;

type PageError = Box<dyn std::error::Error>;

pub enum PageReaderProtocol {
    HTTP,
    TCP
}

impl PageReaderProtocol {
    pub fn create_reader(&self) -> Box<&'static dyn PageReader> {
        match self {
            PageReaderProtocol::HTTP => Box::new(&HTTPReader{}),
            PageReaderProtocol::TCP => Box::new(&TCPReader{})
        }
    }
}

impl std::str::FromStr for PageReaderProtocol {
    type Err = ();

    fn from_str(input: &str) -> Result<PageReaderProtocol, Self::Err> {
        match input {
            "http" => Ok(PageReaderProtocol::HTTP),
            "tcp" => Ok(PageReaderProtocol::TCP),
            _ => Err(()),
        }
    }
}

pub trait PageReader {
    fn read_page(&self, url: &str) -> Result<String, PageError>;
}

pub struct HTTPReader {}

impl PageReader for HTTPReader {
    fn read_page(&self, url: &str) -> Result<String, PageError> {
        let mut response = reqwest::get(format!("http://{}", url).as_str())?;
        let content = response.text()?;
        Ok(content)
    }
}

pub struct TCPReader {}

impl PageReader for TCPReader {
    fn read_page(&self, url: &str) -> Result<String, PageError> {
        let mut conn = TcpStream::connect(format!("{}:80", url))?;
        conn.write_all(b"GET / HTTP/1.0")?;
        conn.write_all(b"\r\n")?;
        conn.write_all(format!("Host: {}", url).as_bytes())?;
        conn.write(b"\r\n\r\n")?;

        let mut buf = vec![];
        let mut content = std::io::Cursor::new(&mut buf);
        std::io::copy(&mut conn, &mut content)?;

        Ok(String::from_utf8(buf)?)
    }
}
