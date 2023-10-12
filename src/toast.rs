use rodio::{source::Source, Decoder, OutputStream};
use std::fs::File;
use std::io::BufReader;
// Supports W/L
use notify_rust::Notification;

#[cfg(target_os = "windows")]
use std::path::Path;
#[cfg(target_os = "windows")]
use winrt_notification::{Duration, Toast};
#[cfg(target_os = "windows")]
use winrt_toast::register;

pub fn gain() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    // Load a sound from a file, using a path relative to Cargo.toml
    let sound_path = if cfg!(target_os = "windows") {
        "C:\\opt\\sentinel\\media\\sound\\gain.wav"
    } else {
        "/opt/sentinel/media/sound/gain.wav"
    };
    let file = BufReader::new(File::open(sound_path).unwrap());
    // Decode that sound file into a source
    let source = Decoder::new(file).unwrap();
    // Play the sound directly on the device
    let _ = stream_handle.play_raw(source.convert_samples());

    if cfg!(target_os = "windows") {
        let _ = register(
            "SENTINEL",
            "Sentinel",
            Some(Path::new("C:\\opt\\sentinel\\media\\logo.ico")),
        );

        Toast::new("SENTINEL")
            .title("Points Earned")
            .text1("You Earned Points!")
            .sound(None)
            .duration(Duration::Short)
            .icon(
                Path::new("C:\\opt\\sentinel\\media\\logo.ico"),
                winrt_notification::IconCrop::Circular,
                "Sentinel",
            )
            .show()
            .unwrap();
    } else if cfg!(target_os = "linux") {
        Notification::new()
            .summary("Points Earned")
            .body("You Earned Points!")
            .icon("/opt/sentinel/media/logo.ico")
            .appname("Sentinel")
            .show()
            .unwrap();
    }

    std::thread::sleep(std::time::Duration::from_secs(5));
}

pub fn loss() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    // Load a sound from a file, using a path relative to Cargo.toml
    let sound_path = if cfg!(target_os = "windows") {
        "C:\\opt\\sentinel\\media\\sound\\alarm.wav"
    } else {
        "/opt/sentinel/media/sound/alarm.wav"
    };
    let file = BufReader::new(File::open(sound_path).unwrap());
    // Decode that sound file into a source
    let source = Decoder::new(file).unwrap();
    // Play the sound directly on the device
    let _ = stream_handle.play_raw(source.convert_samples());

    if cfg!(target_os = "windows") {
        let _ = register(
            "SENTINEL",
            "Sentinel",
            Some(Path::new("C:\\opt\\sentinel\\media\\logo.ico")),
        );

        Toast::new("SENTINEL")
            .title("Points Lost")
            .text1("You Lost Points!")
            .sound(None)
            .duration(Duration::Short)
            .icon(
                Path::new("C:\\opt\\sentinel\\media\\logo.ico"),
                winrt_notification::IconCrop::Circular,
                "Sentinel",
            )
            .show()
            .unwrap();
    } else if cfg!(target_os = "linux") {
        Notification::new()
            .summary("Points Lost")
            .body("You Lost Points!")
            .icon("/opt/sentinel/media/logo.ico")
            .appname("Sentinel")
            .show()
            .unwrap();
    }

    std::thread::sleep(std::time::Duration::from_secs(5));
}
