fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .compile(
            &[
                "proto/axon_server/command.proto",
                "proto/axon_server/control.proto",
                "proto/axon_server/event.proto",
                "proto/axon_server/query.proto",
                "proto/axon_server/common.proto",
            ],
            &["proto/axon_server"],
        )?;
    Ok(())
}
