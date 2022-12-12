use futures_util::stream::StreamExt;
use marek_google_speech_recognition::GoogleRecognizerFactory;
use marek_speech_recognition_api::{Recognizer, RecognizerFactory, RecognizerOptions};
use std::error::Error;
use std::thread::sleep;
use std::time::Duration;
use std::{env, future};
use tokio::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let file = load_audio_data().await?;
    let audio_raw_data = &file[0x2C..];

    let mut recognizer_factory = GoogleRecognizerFactory::new(".", "./SODALanguagePacks")?;
    let (mut recognizer, event_receiver) =
        recognizer_factory.create_recognizer(RecognizerOptions::default())?;

    recognizer.start()?;

    tokio::spawn(event_receiver.for_each(|ev| {
        println!("Event: {:?}", ev);
        future::ready(())
    }));

    let step_size = 2048;
    for pos in (0..audio_raw_data.len()).step_by(step_size) {
        sleep(Duration::from_millis(
            ((step_size as u64 * 1000u64) / (16000u64 * 2u64)) as u64,
        ));
        recognizer.write(&audio_raw_data[pos..(pos + step_size).min(audio_raw_data.len())])?;
    }

    recognizer.stop()?;

    Ok(())
}

async fn load_audio_data() -> Result<Vec<u8>, Box<dyn Error>> {
    let mut path = env::current_dir()?;
    path.push("data");
    path.push("whatstheweatherlike.wav");
    let file = fs::read(path).await?;
    Ok(file)
}
