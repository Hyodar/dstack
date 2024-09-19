fn main() {
    let out_dir = "./src/generated";

    let mut builder = prpc_build::configure()
        .out_dir(out_dir)
        .mod_prefix("super::")
        .disable_package_emission();
    builder = builder.type_attribute(".kms", "#[::prpc::serde_helpers::prpc_serde_bytes]");
    builder = builder.type_attribute(
        ".kms",
        "#[derive(::serde::Serialize, ::serde::Deserialize)]",
    );
    builder = builder.field_attribute(".kms", "#[serde(default)]");
    builder
        .compile(&["kms_rpc.proto"], &["./proto"])
        .expect("failed to compile proto files");
}