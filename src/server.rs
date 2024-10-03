// use tokio::net::TcpListener;
// use tokio::io::{AsyncReadExt, AsyncWriteExt};

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let listener = TcpListener::bind("127.0.0.1:8080").await?;

//     loop {
//         let (mut socket, _) = listener.accept().await?;
//         tokio::spawn(async move {
//             let mut buffer = [0; 1024];

//             match socket.read(&mut buffer).await {
//                 Ok(_) => {
//                     let response = "HTTP/1.1 200 OK\r\n\r\nHello, World!";
//                     let _ = socket.write_all(response.as_bytes()).await;
//                 }
//                 Err(e) => println!("failed to read from socket; err = {:?}", e),
//             }
//         });
//     }
// }