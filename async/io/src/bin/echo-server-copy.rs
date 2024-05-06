use tokio::io;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6142").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            // We only have a single TcpStream. This single value implements both AsyncRead and AsyncWrite. 
            // Because io::copy requires &mut for both the reader and the writer, the socket cannot be used for both arguments
            // This fails to compile
            // io::copy(&mut socket, &mut socket).await
            // TcpStream::split takes a reference to the stream and returns a reader and writer handle. 
            // Because a reference is used, both handles must stay on the same task that split() was called from. This specialized split is zero-cost
            let (mut rd, mut wr) = socket.split();
    
            if io::copy(&mut rd, &mut wr).await.is_err() {
                eprintln!("failed to copy");
            }
        });
    }
}