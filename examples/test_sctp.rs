//! A simple example demonstrating use of the APIs
//! Sends a Ping to the server
//!

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let server_address: std::net::SocketAddr = "127.0.0.1:2353".parse().unwrap();

    let client_socket = sctp_rs::Socket::new_v4(sctp_rs::SocketToAssociation::OneToOne)?;

    let (connected, assoc_id) = client_socket.sctp_connectx(&[server_address]).await?;
    eprintln!("conected: {:#?}, assoc_id: {}", connected, assoc_id);

    for i in 0..10 {
        let message = format!("sctp-rs ping : {}", i);
        let send_data = sctp_rs::SendData {
            payload: message.as_bytes().to_vec(),
            snd_info: None,
        };
        connected.sctp_send(send_data).await?;
        let received = connected.sctp_recv().await?;
        eprintln!("received: {:#?}", received);
    }

    Ok(())
}
