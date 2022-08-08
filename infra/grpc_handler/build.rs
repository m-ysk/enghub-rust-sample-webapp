fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure().compile(TARGET_PROTO, INCLUDE)?;
    Ok(())
}

const TARGET_PROTO: &[&str] = &["../../proto/user/v1/user.proto"];
const INCLUDE: &[&str] = &["../../proto"];
