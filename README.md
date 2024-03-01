# Minimal AWS SDK EC2

Only keep the following APIs

- create_vpc_endpoint
- delete_vpc_endpoints
- describe_vpc_endpoints
- describe_vpc_endpoint_services
- describe_subnets

## How is this done?

Copy the original version

```
cp -R ~/.cargo/registry/src/index.crates.io-6f17d22bba15001f/aws-sdk-ec2-1.2.0/src .
cp ~/.cargo/registry/src/index.crates.io-6f17d22bba15001f/aws-sdk-ec2-1.2.0/Cargo.toml .
```

1. Modify `client.rs` and `operation.rs` to only keep the required `mod`s.
2. Update `protocol_serde.rs` and `types.rs`. This need to contain not only the required APIs, but also some other depended types. So manual work is needed.
