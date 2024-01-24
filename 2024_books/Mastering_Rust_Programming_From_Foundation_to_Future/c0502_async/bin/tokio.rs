use tokio::{
    fs,
    io::AsyncWriteExt,
};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    fs::create_dir_all("data").await?;

    let mut file = fs::File::create("data/tokio_a01.txt").await?;
    file.write_all(b"Hello, Tokio World!\n").await?;

	let contents = fs::read_to_string("data/tokio_a01.txt").await?;
	println!("~~~ contents: {:?}", contents);

	Ok(())
}
