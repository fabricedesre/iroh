fn main() {
    let mut config = prost_build::Config::new();
    config.bytes(&[
        ".p2p.BitswapResponse",
        ".gossipsub.PublishRequest.data",
        ".store.PutRequest.blob",
        ".store.GetResponse.data",
    ]);

    tonic_build::configure()
        .compile_with_config(
            config,
            &[
                "proto/p2p.proto",
                "proto/store.proto",
                "proto/gossipsub.proto",
                "proto/gateway.proto",
                "proto/test.proto",
            ],
            &["proto"],
        )
        .unwrap();
}
