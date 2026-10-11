use std::error::Error;
use env_logger::Builder;
use log::LevelFilter;
use mwave_fm1_utils::FM1Player;

fn main() -> Result<(), Box<dyn Error>> {
    Builder::new().filter_level(LevelFilter::Debug).try_init()?;

    let mut fm1_player = FM1Player::new()?;


    Ok(())
}