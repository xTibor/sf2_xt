mod error;
mod sf2_info;
mod sf2_instrument;
mod sf2_preset_header;
mod sf2_sample;
mod sf2_soundfont;
mod sf2_version;
mod utils;

pub use error::Sf2Error;
pub use sf2_soundfont::Sf2SoundFont;

pub use sf2_info::Sf2Info;
pub use sf2_instrument::Sf2Instrument;
pub use sf2_preset_header::Sf2PresetHeader;
pub use sf2_sample::Sf2Sample;
pub use sf2_version::Sf2Version;

pub type Sf2Result<T> = Result<T, Sf2Error>;
