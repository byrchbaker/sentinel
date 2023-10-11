use notify_rust::Notification;
use rodio::{source::Source, Decoder, OutputStream};
use std::fs::File;
use std::io::BufReader;

pub fn toast(category: &str) {
    match category {
        "gain" => {
            let (_stream, stream_handle) = OutputStream::try_default().unwrap();
            // Load a sound from a file, using a path relative to Cargo.toml
            let file = BufReader::new(File::open("/opt/sentinel/media/sound/gain.wav").unwrap());
            // Decode that sound file into a source
            let source = Decoder::new(file).unwrap();
            // Play the sound directly on the device
            let _ = stream_handle.play_raw(source.convert_samples());

            Notification::new()
                .summary("Points Earned")
                .body("You Earned Points!")
                .icon("/opt/sentinel/media/logo.ico")
                .show()
                .unwrap();

            std::thread::sleep(std::time::Duration::from_secs(5));
        }
        "lost" => {
            let (_stream, stream_handle) = OutputStream::try_default().unwrap();
            // Load a sound from a file, using a path relative to Cargo.toml
            let file = BufReader::new(File::open("/opt/sentinel/media/sound/alarm.wav").unwrap());
            // Decode that sound file into a source
            let source = Decoder::new(file).unwrap();
            // Play the sound directly on the device
            let _ = stream_handle.play_raw(source.convert_samples());

            Notification::new()
                .summary("Points Lost")
                .body("You Lost Points!")
                .icon("/opt/sentinel/media/logo.ico")
                .show()
                .unwrap();

            std::thread::sleep(std::time::Duration::from_secs(5));
        }
        "siren" => {
            let (_stream, stream_handle) = OutputStream::try_default().unwrap();
            // Load a sound from a file, using a path relative to Cargo.toml
            let file = BufReader::new(File::open("/opt/sentinel/media/sound/siren.wav").unwrap());
            // Decode that sound file into a source
            let source = Decoder::new(file).unwrap();
            // Play the sound directly on the device
            let _ = stream_handle.play_raw(source.convert_samples());

            Notification::new()
                .summary("Engine Status")
                .body("Please enter your unique identifier!")
                .icon("/opt/sentinel/media/logo.ico")
                .show()
                .unwrap();

            std::thread::sleep(std::time::Duration::from_secs(5));
        }
        "gong" => {
            let (_stream, stream_handle) = OutputStream::try_default().unwrap();
            // Load a sound from a file, using a path relative to Cargo.toml
            let file = BufReader::new(File::open("/opt/sentinel/media/sound/gong.wav").unwrap());
            // Decode that sound file into a source
            let source = Decoder::new(file).unwrap();
            // Play the sound directly on the device
            let _ = stream_handle.play_raw(source.convert_samples());

            Notification::new()
                .summary("Time Alert")
                .body("Your competition time is running out!")
                .icon("/opt/sentinel/media/logo.ico")
                .show()
                .unwrap();

            std::thread::sleep(std::time::Duration::from_secs(5));
        }
        "horn" => {
            let (_stream, stream_handle) = OutputStream::try_default().unwrap();
            // Load a sound from a file, using a path relative to Cargo.toml
            let file = BufReader::new(File::open("/opt/sentinel/media/sound/horn.wav").unwrap());
            // Decode that sound file into a source
            let source = Decoder::new(file).unwrap();
            // Play the sound directly on the device
            let _ = stream_handle.play_raw(source.convert_samples());

            Notification::new()
                .summary("Time Alert")
                .body("Your team has exceeded the competition time.")
                .icon("/opt/sentinel/media/logo.ico")
                .show()
                .unwrap();

            std::thread::sleep(std::time::Duration::from_secs(5));
        }
        _ => println!("Input does not equal any value"),
    }
}
