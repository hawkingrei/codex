use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rustc-check-cfg=cfg(codex_bazel)");
    println!("cargo:rerun-if-changed=src/grpc");

    // AGENTHUB PATCH: prefer an explicit PROTOC override (e.g. a Bazel-provided system
    // protoc) over the vendored binary. protoc-bin-vendored's bundled binary is located
    // via CARGO_MANIFEST_DIR at runtime, which Bazel's crate_universe sandboxing doesn't
    // reliably propagate across crate boundaries for build-script dependencies.
    let protoc_path = match std::env::var_os("PROTOC") {
        Some(path) => PathBuf::from(path),
        None => protoc_bin_vendored::protoc_bin_path()?,
    };
    let mut config = tonic_prost_build::Config::new();
    config.protoc_executable(protoc_path);
    let proto_files = glob::glob("src/grpc/*.proto")?.collect::<Result<Vec<_>, _>>()?;

    tonic_prost_build::configure()
        .build_client(/*enable*/ true)
        .build_server(/*enable*/ true)
        .compile_with_config(config, &proto_files, &[PathBuf::from("src/grpc")])?;

    Ok(())
}
