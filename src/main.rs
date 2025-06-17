use std::net::SocketAddr;
use std::time::{Duration, Instant};
use tokio::net::UdpSocket;
use once_cell::sync::Lazy;
use dashmap::DashMap;
use tokio::time::sleep;


static CONNECTIONS: Lazy<DashMap<SocketAddr, Instant>> = Lazy::new(DashMap::new);


#[tokio::main]
async fn main() {
    let socket = UdpSocket::bind("0.0.0.0:83").await.unwrap();

    tokio::spawn(async {
        remove_dead_connections().await;
    });

    let mut buf = [0u8; 512];

    loop {
        let (len, addr) = match socket.recv_from(&mut buf).await {
            Ok(res) => res,
            Err(err) => {
                println!("Receive error: {}", err);
                continue; 
            }
        };
        CONNECTIONS.insert(addr, Instant::now());

        println!("Received '{}' from {}. Current connection count: {}", String::from_utf8_lossy(&buf[..len]), addr, CONNECTIONS.len());

        for address in CONNECTIONS.iter() {
            if *address.key() == addr {
                continue;
            }
            
            let msg = format!("{}|{}", addr, String::from_utf8_lossy(&buf[..len]));

            if let Err(e) = socket.send_to(msg.as_bytes(), address.key()).await {
                eprintln!("Failed to send to {}: {}", address.key(), e);
            }
        }
    }
}


async fn remove_dead_connections() {
    loop {
        let now = Instant::now();
        let inactive: Vec<_> = CONNECTIONS
            .iter()
            .filter(|entry| now.duration_since(*entry.value()) > Duration::from_secs(30))
            .map(|entry| *entry.key())
            .collect();

        for key in inactive {
            CONNECTIONS.remove(&key);
            println!("Removed inactive: {}", key);
        }

        sleep(Duration::from_secs(60)).await;
    }
}
