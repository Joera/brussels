use std::process::Command;

fn main() {
    if Command::new("ipfs").arg("--version").status().is_err() {
        println!("cargo::warning=Error: 'ipfs' command not found. Make sure it is installed and available in your system's PATH.");
        std::process::exit(1);
    }

    let out_dir = std::env::var("OUT_DIR").expect("OUT_DIR not set");
    let target_cid_file = "cidv1";
    let wasm_path = "../target/wasm32-wasi/release/lotus_effector.wasm";

    println!("cargo:rerun-if-changed={}", wasm_path);

    let output = Command::new("ipfs")
        .args(&[
            "add",
            "--only-hash",
            "-Q",
            "--cid-version",
            "1",
            "--hash",
            "sha2-256",
            "--chunker=size-262144",
            wasm_path,
        ])
        .output()
        .expect("Failed to execute IPFS command");

    if output.status.success() {
        let cid = String::from_utf8(output.stdout)
            .unwrap_or_else(|e| format!("Failed to parse IPFS output: {e:?}"));
        let cid = cid.trim();
        let cid_path = format!("{}/{}", out_dir, target_cid_file);
        println!("cargo:warning=Generated CIDv1 {}", cid);
        std::fs::write(&cid_path, cid).expect("Failed to write CID to file");
    } else {
        panic!("Failed to compute CIDv1 of wasm file {wasm_path} with IPFS: {output:?}");
    }

    built::write_built_file().expect("Failed to acquire build-time information")
}
