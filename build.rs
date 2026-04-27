fn main() -> Result <(), Box<dyn std::error::Error>> {
    tonic_build::configrue()
        .build_server(true)
        .compile(
            &["proto/services.proto"],
            &["proto"],
        )?;
    Ok(())
}
