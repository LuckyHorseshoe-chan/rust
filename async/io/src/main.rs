use tokio::fs::File;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut f = File::open("foo.txt").await?;
    let mut buffer = [0; 10];

    // read up to 10 bytes
    let n = f.read(&mut buffer[..]).await?;

    // The bytes: [65, 110, 32, 105, 109, 109, 117, 116, 97, 98]
    println!("The bytes: {:?}", &buffer[..n]);

    let mut buffer = Vec::new();

    // read the whole file
    f.read_to_end(&mut buffer).await?;

    let mut f = File::create("foo1.txt").await?;
    // Writes some prefix of the byte string, but not necessarily all of
    let n = f.write(b"some bytes").await?;

    println!("Wrote the first {} bytes of 'some bytes'.", n);
    
    // f.write_all(b"some bytes").await?;

    let mut reader: &[u8] = b"hello";
    let mut file = File::create("foo2.txt").await?;

    io::copy(&mut reader, &mut file).await?;
    Ok(())
}