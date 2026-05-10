// tonic-build invokes protoc at compile time to generate the
// `sentrix.v1` Rust module from `proto/sentrix.proto`. We disable
// server codegen — this crate is browser-only, no need for the
// service stub.

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(false)
        .compile_protos(&["proto/sentrix.proto"], &["proto"])?;
    Ok(())
}
