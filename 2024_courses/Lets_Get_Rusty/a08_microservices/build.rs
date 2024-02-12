use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    tonic_build::compile_protos("proto/hello.proto")?;

    tonic_build::configure()
        .build_server(true)
        .compile(&["proto/authentication.proto"], &["proto/authentication"])?;

    Ok(())
}
