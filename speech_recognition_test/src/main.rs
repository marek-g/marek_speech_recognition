use byteorder::{ByteOrder, LittleEndian};
use futures_util::stream::StreamExt;
use marek_google_speech_recognition::GoogleRecognizerFactory;
use marek_speech_recognition_api::{
    RecognitionEvent, Recognizer, RecognizerFactory, RecognizerOptions, SpeechResult,
};
use marek_vosk_speech_recognition::{VoskModelInfo, VoskRecognizerFactory};
use std::error::Error;
use std::io::Write;
use std::path::PathBuf;
use std::thread::sleep;
use std::time::Duration;
use std::{env, future};
use tokio::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let audio_raw_data = load_audio_data().await?;

    let mut recognizer_factory = create_recognizer_factory()?;
    let (mut recognizer, event_receiver) =
        recognizer_factory.create_recognizer(RecognizerOptions::default())?;

    recognizer.start()?;

    tokio::spawn(event_receiver.for_each(|ev| {
        //println!("Event: {:?}", ev);
        if let RecognitionEvent::Recognition {
            text,
            is_final,
            audio_start_time_usec,
            audio_end_time_usec,
            words,
        } = ev
        {
            println!("{}: {}", if is_final { "Final" } else { "Partial" }, text);
        } else {
            println!("Event: {:?}", ev);
        }

        future::ready(())
    }));

    let step_size = 1024;
    for pos in (0..audio_raw_data.len()).step_by(step_size) {
        sleep(Duration::from_millis(
            ((step_size as u64 * 1000u64) / 16000u64) as u64,
        ));
        recognizer.write(&audio_raw_data[pos..(pos + step_size).min(audio_raw_data.len())])?;
    }

    recognizer.stop()?;

    sleep(Duration::from_millis(2000));

    Ok(())
}

fn create_recognizer_factory() -> SpeechResult<Box<dyn RecognizerFactory>> {
    let mut answer = String::new();

    loop {
        print!("Choose recognizer [google, vosk]: ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut answer).unwrap();
        answer = answer
            .strip_suffix("\r\n")
            .or(answer.strip_suffix("\n"))
            .unwrap_or(&answer)
            .to_string();

        println!("You chosen: {}", answer);

        if answer == "google" {
            return Ok(Box::new(GoogleRecognizerFactory::new(
                ".",
                "./SODALanguagePacks",
            )?));
        } else if answer == "vosk" {
            return Ok(Box::new(VoskRecognizerFactory::new(vec![VoskModelInfo {
                language: "en-US".to_string(),
                folder: PathBuf::from("/usr/local/share/vosk-models/small-en-us"),
            }])?));
        }
    }
}

async fn load_audio_data() -> Result<Vec<i16>, Box<dyn Error>> {
    let mut path = env::current_dir()?;
    path.push("data");
    path.push("whatstheweatherlike.wav");
    let file = fs::read(path).await?;
    let mut audio_raw_data = vec![0i16; (file.len() - 0x2C) / 2];
    LittleEndian::read_i16_into(&file[0x2C..], &mut audio_raw_data);
    //Ok(audio_raw_data)

    let mut v = Vec::new();
    for _ in 0..3 {
        v.extend_from_slice(&audio_raw_data);
    }
    Ok(v)
}
