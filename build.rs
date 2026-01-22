use std::process::Command;

fn main() {
    for path in ["package.json", "node_modules", "assets", "input.css", "src"] {
        println!("cargo:rerun-if-changed={}", path);
    }

    let status = Command::new("npm")
        .args(["run", "build"])
        .status()
        .expect("Please install npm https://nodejs.org/en/download");

    if !status.success() {
        panic!("Tailwind build failed. Ensure you've run 'npm install'.");
    }
}
