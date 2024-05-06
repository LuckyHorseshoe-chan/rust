use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

// Any reader + writer type can be split using the io::split utility. 
// This function takes a single value and returns separate reader and writer handles. 
#[tokio::main]
async fn main() -> io::Result<()> {
    let socket = TcpStream::connect("127.0.0.1:6142").await?;
    let (mut rd, mut wr) = io::split(socket);
    
    // These two handles can be used independently, including from separate tasks.
    // Write data in the background
    tokio::spawn(async move {
        wr.write_all(b"hello\r\n").await?;
        wr.write_all(b"world\r\n").await?;

        // Sometimes, the rust type inferencer needs
        // a little help
        Ok::<_, io::Error>(())
    });

    let mut buf = vec![0; 128];

    loop {
        let n = rd.read(&mut buf).await?;

        if n == 0 {
            break;
        }

        println!("GOT {:?}", &buf[..n]);
    }

    Ok(())
}

// Because io::split supports any value that implements AsyncRead + AsyncWrite and returns independent handles, internally io::split uses an Arc and a Mutex. 
// This overhead can be avoided with TcpStream