fn main() {
    if std::env::var("CODEGEN_SKIP").is_ok() {
        return;
    }
    let schema_dir = std::path::Path::new("../traO-Judge-docs/api").canonicalize().unwrap();
    println!("cargo:rerun-if-changed={}", schema_dir.to_str().unwrap());
    let out_dir = std::path::Path::new("./src").canonicalize().unwrap();
    let schema_path = std::fs::read_dir(&schema_dir)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .find(|path| path.extension() == Some(std::ffi::OsStr::new("proto")))
        .unwrap();
    tonic_build::configure()
        .build_server(true)
        .build_client(false)
        .out_dir(out_dir)
        .compile_protos(&[schema_path], &[schema_dir])
        .unwrap();
}