use std::{
    fs,
    io::{self, BufRead, BufReader, Write},
    path::Path,
};

fn main() -> Result<(), io::Error> {
    #[cfg(target_os = "windows")]
    println!("==> Hello from Windows!");

    #[cfg(target_os = "macos")]
    println!("==> Hello from macOS!");

    #[cfg(target_os = "linux")]
    println!("==> Hello from Linux!");

    //
    let path = Path::new("data");

    #[cfg(target_os = "windows")]
    let file = path.join("file.txt");

    #[cfg(not(target_os = "windows"))]
    let file = path.join("file.txt");

    println!("==> File path: {:?}", file);

    //
    let fp = "data/file.txt";
    fs::create_dir_all("data")?;

	let mut file = fs::File::create(fp)?;

	#[cfg(target_os = "windows")]
	file.write_all(b"Hello, Rust\r\n")?;

	#[cfg(not(target_os = "windows"))]
	file.write_all(b"Hello, Rust\n")?;

	file.sync_all()?;
	drop(file);

    //
    let file = fs::File::open(fp)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        #[cfg(target_os = "windows")]
        let line = line?.replace("\r", "");

        println!("{:?}", line);
    }

    Ok(())
}
