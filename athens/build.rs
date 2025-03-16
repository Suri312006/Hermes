fn main() {
    tonic_build::configure()
        .build_client(true)
        .protoc_arg("--experimental_allow_proto3_optional")
        .use_arc_self(true)
        // this is where our proto files are
        .compile_protos(
            &[
                "../proto/message.proto",
                "../proto/user.proto",
                "../proto/proxy.proto",
            ],
            &["../proto"],
        )
        .unwrap_or_else(|e| panic!("unable to compile proto's due to {e:?}"))
}
