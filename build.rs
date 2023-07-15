use glob::glob;

fn main() {
    let proto_files: Vec<_> = glob("protos/*.proto")
        .expect("Failed to read glob pattern")
        .filter_map(Result::ok)
        .map(|path| path.to_string_lossy().into_owned())
        .collect();

    tonic_build::configure()
        .out_dir("pb")
        .build_server(true)
        .build_client(false)
        .file_descriptor_set_path("pb/reflection_descriptor.bin")
        .compile(&proto_files, &["protos"])
        .unwrap_or_else(|e| panic!("Failed to compile protos {:?}", e));
}
