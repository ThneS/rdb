use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

async fn process(mut stream: TcpStream) {
    let mut buf = [0; 1024];
    let length = stream.read_exact(&mut buf).await.unwrap();
    let write_len = stream.write_all(&buf).await.unwrap();
    println!("length: {:?}, {:?}, {:?}", length, buf, write_len);
}