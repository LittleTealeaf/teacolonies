use std::path::Path;

use teacolonies::core::{crafters::Mechanic, datapack::DataPack, recipe::Recipe};
use teacolonies::datapack::{BrewingModule, CarpentryModule, DyerModule};

fn main() {
    let mut dp = DataPack::new("teacolonies");

    dp.apply::<BrewingModule>();
    dp.apply::<DyerModule>();
    dp.apply::<CarpentryModule>();

    dp.save_to_zip(Path::new("teacolonies.zip")).unwrap();
}
