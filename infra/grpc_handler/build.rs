fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::fs::create_dir_all(TARGET_DIR)?;
    tonic_build::configure()
        .out_dir(TARGET_DIR)
        .compile(TARGET_PROTO, INCLUDE)?;
    Ok(())
}

const TARGET_PROTO: &[&str] = &["../../proto/user/v1/user.proto"];
const INCLUDE: &[&str] = &["../../proto"];
const TARGET_DIR: &str = "generated/proto";
