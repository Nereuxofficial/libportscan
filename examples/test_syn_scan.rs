use socket2::{Domain, Protocol, SockAddr, Socket, Type};
use std::{error::Error, io::Write, net::SocketAddr, str::FromStr};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
   let mut socket = Socket::new(Domain::IPV4, Type::RAW, Some(Protocol::TCP))?;
   socket.set_nonblocking(true)?;
   let addr = SocketAddr::from_str("127.0.0.1:8080");
   socket.connect(addr.into())?;
   let source_port = socket.local_addr()?.port();
   let source_addr = socket.local_addr()?;
   let dest_port = addr.port();
   let dest_addr = addr.ip();
   // Generate the right TCP SYN packet
   let packet = generate_syn_packet(source_addr, addr);
   socket.send(&packet)?;
   Ok(()) 
}