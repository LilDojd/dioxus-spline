use std::process::Command;

fn main() {
    // npminstall
    let output = Command::new("npm")
        .args(["install"])
        .output()
        .expect("Failed to install @splinetool/runtime");

    println!(
        "npm install output: {:?}",
        String::from_utf8_lossy(&output.stdout)
    );

    // Run npm run esbuild
    let output = Command::new("npm")
        .args(["run", "esbuild"])
        .output()
        .expect("Failed to run npm run esbuild");
    println!(
        "npm run esbuild output: {:?}",
        String::from_utf8_lossy(&output.stdout)
    );

    // Proceed with the Rust compilation
    println!("cargo:rerun-if-changed=node_modules/@splinetool/package.json");
}
