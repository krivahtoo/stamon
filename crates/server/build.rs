use std::process::Command;

fn main() {
    // trigger recompilation when a new migration is added
    println!("cargo:rerun-if-changed=migrations");

    // Set version env
    if let Ok(output) = Command::new("git").args(["describe", "--tags"]).output() {
        if output.status.success() {
            println!(
                "cargo:rustc-env=STAMON_VERSION={}",
                String::from_utf8_lossy(&output.stdout)
            );
        }
    }
}
