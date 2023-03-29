use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .out_dir("src")
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
    fs::remove_file("src/google.protobuf.rs").ok();
    fs::create_dir_all("src/axon_server").ok();
    fs::rename(
        "src/io.axoniq.axonserver.grpc.common.rs",
        "src/axon_server/common.rs",
    )?;
    fs::rename(
        "src/io.axoniq.axonserver.grpc.command.rs",
        "src/axon_server/command.rs",
    )?;
    fs::rename(
        "src/io.axoniq.axonserver.grpc.control.rs",
        "src/axon_server/control.rs",
    )?;
    fs::rename(
        "src/io.axoniq.axonserver.grpc.event.rs",
        "src/axon_server/event.rs",
    )?;
    fs::rename(
        "src/io.axoniq.axonserver.grpc.query.rs",
        "src/axon_server/query.rs",
    )?;
    Ok(())
}
