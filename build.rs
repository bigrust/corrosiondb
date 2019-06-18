extern crate protoc_rust;

use protoc_rust::Customize;

fn main() {
    protoc_rust::run(protoc_rust::Args {
        out_dir: "src/io/",
        includes: &[],
        input: &["proto/coordinator.proto"],
        customize: Customize {
            ..Default::default()
        },
    }).expect("Failed to compile proto files");
}