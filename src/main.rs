use std::path::Path;

use teacolonies::core::datapack::DataPack;
use teacolonies::datapack::{
    BrewingModule, CarpentryModule, DyerModule, MiscModule, NetherModule, StoneModule,
};

#[cfg(debug_assertions)]
use teacolonies::util::{delete_directory_recursive, unzip};

fn main() -> std::io::Result<()> {
    let mut dp = DataPack::new("teacolonies");

    dp.apply::<BrewingModule>();
    dp.apply::<DyerModule>();
    dp.apply::<CarpentryModule>();
    dp.apply::<MiscModule>();
    dp.apply::<NetherModule>();
    dp.apply::<StoneModule>();

    dp.save_to_zip(Path::new("teacolonies.zip"))?;

    #[cfg(debug_assertions)]
    {
        delete_directory_recursive(Path::new("teacolonies"))?;
        unzip("teacolonies.zip", "teacolonies")?;
    }

    Ok(())
}
