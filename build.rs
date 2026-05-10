// tonic-build invokes protoc at compile time to generate the
// `sentrix.v1` Rust module from `proto/sentrix.proto`. We disable
// server codegen — this crate is browser-only, no need for the
// service stub.

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // proto/sentrix.proto uses proto3 `optional` (eg
    // GetValidatorSetRequest.at_height). protoc 3.15+ accepts this by
    // default; ubuntu-22.04 still ships 3.12.4 in apt and needs the
    // flag. Harmless on newer protoc.
    tonic_build::configure()
        .build_server(false)
        .protoc_arg("--experimental_allow_proto3_optional")
        .compile_protos(&["proto/sentrix.proto"], &["proto"])?;
    Ok(())
}
