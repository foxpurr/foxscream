use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=../protobuf/");

    unsafe {
        env::set_var("PROTOC", protoc_bin_vendored::protoc_bin_path()?);
    }

    prost_build::compile_protos(
        &["../protobuf/client.proto", "../protobuf/server.proto"],
        &["../protobuf/"],
    )?;

    Ok(())
}
