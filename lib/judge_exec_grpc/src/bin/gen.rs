#[cfg(feature = "codegen")]
/// Run `cargo run --features codegen --bin gen` to generate.
fn main() {
    let schema_dir = std::path::Path::new("proto").canonicalize().unwrap();
    let out_dir = std::path::Path::new("src/generated")
        .canonicalize()
        .unwrap();
    let schema_path = schema_dir.join("execute.proto");
    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .out_dir(&out_dir)
        .compile_protos(&[schema_path], &[schema_dir])
        .unwrap();
}

#[cfg(not(feature = "codegen"))]
/// Run `cargo run --features codegen --bin gen` to generate.
fn main() {
    panic!("codegen feature is disabled");
}
