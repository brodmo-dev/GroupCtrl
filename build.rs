use std::fs;
use std::process::Command;

const INSTALL_STAMP: &str = "node_modules/.install-stamp";

fn main() {
    for path in ["package.json", "package-lock.json", "input.css", "src"] {
        println!("cargo:rerun-if-changed={}", path);
    }
    if gt_mtime("package-lock.json", INSTALL_STAMP) {
        npm(&["install"]);
        fs::File::create(INSTALL_STAMP).unwrap();
    }
    npm(&["run", "build"]);
}

fn npm(args: &[&str]) {
    let output = Command::new("npm")
        .args(args)
        .output()
        .map_err(|e| {
            // Simulate missing npm using PATH=~/.cargo/bin:/usr/bin cargo check
            panic!(
                "npm not found, please install it: https://nodejs.org/en/download\nerror: {}",
                e
            )
        })
        .unwrap();

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        panic!("command 'npm {}' failed:\n{}", args.join(" "), stderr);
    }
}

fn gt_mtime(left: &str, right: &str) -> bool {
    fn get_mtime(path: &str) -> std::io::Result<std::time::SystemTime> {
        fs::metadata(path)?.modified()
    }
    match (get_mtime(left), get_mtime(right)) {
        (Ok(left_mtime), Ok(right_mtime)) => left_mtime > right_mtime,
        _ => true,
    }
}
