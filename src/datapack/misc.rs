use crate::{
    core::{
        crafters::{Baker, Fletcher, Mechanic},
        datapack::DataPack,
        module::Module,
        recipe::Recipe,
    },
    item, m,
};

pub struct MiscModule;

impl Module for MiscModule {
    fn apply(pack: &mut DataPack) {
        pack.add_recipes([
            Recipe::new(
                Mechanic::Crafting,
                [item!(m!("gold_ingot"), 4), item!(m!("iron_ingot"))],
                item!(m!("bell")),
            ),
            Recipe::new(
                Fletcher::Crafting,
                item!(m!("string"), 3),
                item!(m!("cobweb")),
            ),
            Recipe::new(
                Baker::Crafting,
                item!(m!("bucket")),
                item!(m!("water_bucket")),
            ),
        ]);
    }
}
