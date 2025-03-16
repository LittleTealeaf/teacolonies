use std::path::Path;

use teacolonies::core::{crafters::Mechanic, datapack::DataPack, recipe::Recipe};
use teacolonies::datapack::{BrewingModule, DyerModule};

fn main() {
    let mut dp = DataPack::new("teacolonies");

    // dp.register_recipe(
    //     Recipe::new(
    //         Mechanic::Crafting,
    //         [item!(m!("water_bucket")), item!(m!("lava_bucket"))],
    //         item!(m!("cobblestone"), 64),
    //     )
    //     .with_tool(mc!("pickaxe")),
    // );

    // dp.add_brewing();
    //

    dp.apply::<BrewingModule>();
    dp.apply::<DyerModule>();

    dp.save_to_zip(Path::new("teacolonies.zip")).unwrap();
}
