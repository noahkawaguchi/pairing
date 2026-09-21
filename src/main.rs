use std::{
    collections::HashMap,
    io::{BufRead, BufReader, BufWriter, Write},
    net::{TcpListener, TcpStream},
};

type Result<T = (), E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

fn main() -> Result {
    const BIND_ADDR: &str = "localhost:4000";
    let listener = TcpListener::bind(BIND_ADDR)?;
    println!("Listening on {BIND_ADDR}");

    let mut server = Server::default();

    for stream in listener.incoming() {
        server.handle(stream?)?;
    }

    Ok(())
}

#[derive(Default)]
struct Server {
    map: HashMap<String, String>,
}

impl Server {
    fn handle(&mut self, stream: TcpStream) -> Result {
        let mut reader = BufReader::new(stream.try_clone()?);
        let mut writer = BufWriter::new(stream);

        let mut buf = String::new();
        reader.read_line(&mut buf)?;
        print!("{buf}");

        match Request::parse(&buf).ok_or_else(|| format!("Failed to parse line: {buf}"))? {
            Request::Set(k, v) => {
                self.map.insert(k, v);
                writer.write_all(b"Successfully set\n")?;
            }

            Request::Get(k) => {
                writer.write_all(
                    self.map
                        .get(&k)
                        .map(String::as_bytes)
                        .unwrap_or(b"Not found"),
                )?;

                writer.write_all(b"\n")?;
            }
        }

        Ok(())
    }
}

enum Request {
    Set(String, String),
    Get(String),
}

impl Request {
    fn parse(line: &str) -> Option<Self> {
        // Examples:
        //   GET /set?somekey=someval HTTP/1.1
        //   GET /get?key=somekey HTTP/1.1

        let path = line.split_whitespace().nth(1)?.trim_start_matches('/');
        let (verb, params) = path.split_once('?')?;
        let (param_key, param_val) = params.split_once('=')?;

        match (verb, param_key, param_val) {
            ("set", k, v) => Some(Self::Set(k.to_string(), v.to_string())),
            ("get", "key", k) => Some(Self::Get(k.to_string())),
            _ => None,
        }
    }
}
