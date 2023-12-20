#[cfg(feature = "SOLANA")]
include!("src/solana_config/build_proto.rs");

include!("src/features.rs");

fn main() -> std::io::Result<()> {
    build_protos()?;
    Ok(())
}
