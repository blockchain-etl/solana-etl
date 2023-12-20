use std::path::Path;

pub fn build_protos() -> std::io::Result<()> {
    let config_dir = "src/solana_config/";
    let src_dir = [config_dir, "proto_src/"].concat();
    println!("cargo:rerun-if-changed={}", src_dir);

    let out_dir = [config_dir, "proto_codegen/"].concat();

    if !Path::new(&out_dir).exists() {
        std::fs::create_dir(&out_dir)?;
    }

    let mut config = prost_build::Config::default();

    config
        .message_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .enum_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]");

    config
        .out_dir(out_dir.clone())
        .protoc_arg("--experimental_allow_proto3_optional")
        .compile_protos(
            &[
                [&src_dir, "account_info.proto"].concat(),
                [&src_dir, "confirmed_block.proto"].concat(),
                [&src_dir, "etl_block.proto"].concat(),
                [&src_dir, "transaction_by_addr.proto"].concat(),
            ],
            &[&src_dir],
        )?;

    config.out_dir(out_dir.clone()).compile_protos(
        &[
            [&src_dir, "records_string_timestamp.proto"].concat(),
            [&src_dir, "records_int_timestamp.proto"].concat(),
        ],
        &[&src_dir],
    )?;

    // rust does not allow us to import code from files with multiple .
    std::fs::rename(
        [&out_dir, "solana.account_info.rs"].concat(),
        [&out_dir, "account_info.rs"].concat(),
    )?;
    std::fs::rename(
        [&out_dir, "solana.confirmed_block.rs"].concat(),
        [&out_dir, "confirmed_block.rs"].concat(),
    )?;
    std::fs::rename(
        [&out_dir, "solana.etl_block.rs"].concat(),
        [&out_dir, "etl_block.rs"].concat(),
    )?;
    std::fs::rename(
        [&out_dir, "solana.transaction_by_addr.rs"].concat(),
        [&out_dir, "transaction_by_addr.rs"].concat(),
    )?;
    std::fs::write(
        [&out_dir, "mod.rs"].concat(),
        "pub mod account_info;\npub mod confirmed_block;\npub mod etl_block;\npub mod transaction_by_addr;\n#[cfg(feature=\"INT_TIMESTAMP\")]\npub mod records_int_timestamp;\n#[cfg(feature=\"STRING_TIMESTAMP\")]\npub mod records_string_timestamp;\n",
    )?;
    Ok(())
}
