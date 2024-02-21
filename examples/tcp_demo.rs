use tokio::io::AsyncReadExt;
use tokio::net::{TcpStream, TcpListener};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8989").await.unwrap();
    println!("listening at 127.0.0.1:8989");
    loop {
        let (socket, _) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            process(socket).await
        });
    }
}

async fn process(mut stream: TcpStream) {
    let mut buf = [0; 1024];
    let length = stream.read_exact(&mut buf).await.unwrap();
    println!("length: {:?}, {:?}", length, buf);
}