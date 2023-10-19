use log::error;
use log::info;
use log::warn;
use std::io;

// mod misc;

fn main() {
    env_logger::try_init().unwrap_or_else(|_| println!("Failed to initialize the logger"));

    info!("Starting the worker");

    info!("Installing Rust");
    //let _ = misc::install_rust();

    info!("Have you placed the configuration file in the same directory as the worker? (y/n)");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    if input.trim() != "y" {
        error!("Please place the configuration file in the same directory as the worker");
    } else {
        info!("Configuration file found.")
    }

    info!("Verifying the configuration file");

    //let _ = misc::verify_config();

    info!("Do you want to make sure the engine checks are correct? (y/n)");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    if input.trim() != "y" {
        warn!("Skipping configuration checks");
    } else {
        compile_engine();
        info!("Starting configuration checks for the engine (Open Readme)");
        //let _ = misc::check_config();
    }
}

fn compile_engine() {
    info!("Compiling the engine");
    //let _ = misc::compile_engine();
}
