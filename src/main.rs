use std::path::Path;

use teacolonies::core::datapack::DataPack;
use teacolonies::datapack::{BrewingModule, CarpentryModule, DyerModule, MiscModule, NetherModule};

fn main() {
    let mut dp = DataPack::new("teacolonies");

    dp.apply::<BrewingModule>();
    dp.apply::<DyerModule>();
    dp.apply::<CarpentryModule>();
    dp.apply::<MiscModule>();
    dp.apply::<NetherModule>();

    dp.save_to_zip(Path::new("teacolonies.zip")).unwrap();
}
