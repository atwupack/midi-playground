use std::error::Error;
use mwave_fm1_utils::presets::FM1Presets;

fn main() -> Result<(), Box<dyn Error>> {
    let mut preset = FM1Presets::new()?;
    preset.upload_preset(0, "mwave-fm1-utils/data/fm1-F13-E-CEMBALO.syx")?;
    Ok(())
}