fn main() {
    let pinger_proto = "proto/pinger/pinger.proto";

    tonic_build::compile_protos(pinger_proto).unwrap();
}
