use std::{
    fs::File,
    io,
    io::Write,
    process::{Command, Stdio},
};

#[cfg(target_os = "windows")]
const URL: &str = "https://win.rustup.rs/";

pub fn install_rust() {
    let _ = download_rust_installer();
    let _ = run_installer_silently();
}

#[cfg(target_os = "windows")]
fn download_rust_installer() -> Result<(), Box<dyn std::error::Error>> {
    let mut response = reqwest::blocking::get(URL)?;
    if !response.status().is_success() {
        eprintln!("Failed to download the Rust installer");
        return Ok(());
    }

    let mut installer = File::create("rustup-init.exe")?;
    io::copy(&mut response, &mut installer)?;

    Ok(())
}

#[cfg(target_os = "windows")]
fn run_installer_silently() -> Result<(), Box<dyn std::error::Error>> {
    std::thread::sleep(std::time::Duration::from_secs(2));

    let mut command = Command::new("rustup-init.exe");

    command.args(&["--default-host", "x86_64-pc-windows-msvc"]);
    command.args(&["--default-toolchain", "stable"]);
    command.arg("-y");

    // Redirect the standard input, output, and error to null
    command.stdin(Stdio::null());
    command.stdout(Stdio::null());
    command.stderr(Stdio::null());

    let status = command.status()?;

    if !status.success() {
        eprintln!("Failed to run the Rust installer");
    }

    Ok(())
}

#[cfg(target_os = "linux")]
fn run_installer_silently() {
    // Run the shell command to download and install Rust
    let status = Command::new("sh")
        .arg("-c")
        .arg(r#"curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y"#)
        .status()
        .expect("Failed to execute command");

    if status.success() {
        println!("Rust installation successful!");
    } else {
        eprintln!("Rust installation failed.");
    }
}
