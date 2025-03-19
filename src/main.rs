use std::path::Path;

use teacolonies::core::datapack::DataPack;
use teacolonies::datapack::{BrewingModule, CarpentryModule, DyerModule, MiscModule, NetherModule};
use teacolonies::util::unzip;

fn main() -> std::io::Result<()> {
    let mut dp = DataPack::new("teacolonies");

    dp.apply::<BrewingModule>();
    dp.apply::<DyerModule>();
    dp.apply::<CarpentryModule>();
    dp.apply::<MiscModule>();
    dp.apply::<NetherModule>();

    dp.save_to_zip(Path::new("teacolonies.zip"))?;

    #[cfg(debug_assertions)]
    {
        unzip("teacolonies.zip", "teacolonies")?;
    }

    Ok(())
}
