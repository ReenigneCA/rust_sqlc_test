use std::env;
// build.rs
use std::process::Command;

fn main() {
    if env::var("CARGO_INTELLIJ_RUST_SUPPORT").is_ok() {
        return;
    }

    let mut cmd = Command::new("sqlc");
    let mut fmtcmd = Command::new("./rustfmt.sh");

    cmd.arg("generate");

    let output = cmd.output().expect("failed to run sqlc generate");
    let fmt_output = fmtcmd.output().expect("failed to run rustfmt");
    if !output.status.success() {
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));

        panic!("sqlc failed with exit code: {}", output.status);
    }

    if !fmt_output.status.success() {
        eprintln!("{}", String::from_utf8_lossy(&fmt_output.stderr));

        // Panic to stop the build if the script fails
        panic!("rustfmt.sh failed: {}", fmt_output.status);
    }

    println!("{}", String::from_utf8_lossy(&output.stdout));
}
