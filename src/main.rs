use std::path::Path;

use teacolonies::{crafters::Mechanic, datapack::DataPack, item, m, mc, nbt, recipe::Recipe};

fn main() {
    let mut dp = DataPack::new("teacolonies");

    dp.add_recipe(
        Recipe::new(
            Mechanic::Crafting,
            [item!(m!("water_bucket")), item!(m!("lava_bucket"))],
            item!(m!("cobblestone"), 64),
        )
        .with_tool(mc!("pickaxe")),
    );

    potions::add(&mut dp);

    dp.save_to_zip(Path::new("teacolonies.zip")).unwrap();
}

mod potions {
    use teacolonies::crafters::Alchemist;

    use super::*;

    pub fn add(dp: &mut DataPack) {
        dp.add_recipe(Recipe::new(
            Alchemist::Crafting,
            [item!(m!("ender_pearl"), 16)],
            item!(m!("nether_star")),
        ));

        dp.add_recipe(Recipe::new(
            Alchemist::Crafting,
            [item!(m!("glass_bottle"))],
            item!(nbt!(m!("potion"), "Potion" = "water".to_owned())),
        ));
    }
}
