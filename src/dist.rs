// This file includes the infrastructure for distributing renders across
// multiple machines.
use std::scene::Scene;

struct Client {
    server: TcpStream,
}

impl Client {
    pub fn new(socket: TcpStream) -> io::Result<Client> {
        //
    }

    pub fn job(&mut self) -> io::Result<RenderJob> {
        //
    }
}

struct RenderRunner {
    scenes: LruCache<String, Scene>,
}

impl RenderRunner {
    pub fn new() -> RenderRunner {
        RenderRunner {
            scenes: LruCache::new(2),
        }
    }
}