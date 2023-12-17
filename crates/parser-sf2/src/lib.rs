mod error;
mod sf2_info;
mod sf2_instrument_generator;
mod sf2_instrument_header;
mod sf2_instrument_modulator;
mod sf2_instrument_zone;
mod sf2_preset_generator;
mod sf2_preset_header;
mod sf2_preset_modulator;
mod sf2_preset_zone;
mod sf2_sample_header;
mod sf2_soundfont;
mod sf2_version;
mod utils;

pub use error::Sf2Error;
pub use sf2_soundfont::Sf2SoundFont;

pub use sf2_info::Sf2Info;
pub use sf2_instrument_generator::Sf2InstrumentGenerator;
pub use sf2_instrument_header::Sf2InstrumentHeader;
pub use sf2_instrument_modulator::Sf2InstrumentModulator;
pub use sf2_instrument_zone::Sf2InstrumentZone;
pub use sf2_preset_generator::Sf2PresetGenerator;
pub use sf2_preset_header::Sf2PresetHeader;
pub use sf2_preset_modulator::Sf2PresetModulator;
pub use sf2_preset_zone::Sf2PresetZone;
pub use sf2_sample_header::Sf2SampleHeader;
pub use sf2_version::Sf2Version;

pub type Sf2Result<T> = Result<T, Sf2Error>;
