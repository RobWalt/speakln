use once_cell::sync::Lazy;
use std::sync::Mutex;

use tts::{Backends, Tts};

#[macro_export]
macro_rules! speakln {
    ($x:expr) => {
        SPEAKLN_TTS
            .lock()
            .expect("can only be used in sync modes")
            .speak(format!("{:?}", $x), false)
            .expect("speaking worked");
    };
}

pub static SPEAKLN_TTS: Lazy<Mutex<Tts>> =
    Lazy::new(|| Mutex::new(Tts::new(Backends::SpeechDispatcher).expect("ok")));

pub mod prelude {
    pub use super::SPEAKLN_TTS;
}
