//! Build file generating gRPC stubs

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/tracker_service.proto")?;
    Ok(())
}
