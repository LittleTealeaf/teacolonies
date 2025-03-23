use std::iter::once;

use crate::{
    core::{
        crafters::{Baker, Farmer, Fletcher, Mechanic},
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
            Recipe::new(
                Farmer::Crafting,
                [
                    item!(m!("grass_block")),
                    item!(m!("red_mushroom")),
                    item!(m!("brown_mushroom")),
                ],
                item!(m!("mycelium")),
            ),
            Recipe::new(
                Farmer::Crafting,
                item!(m!("dirt"), 4),
                item!(m!("rooted_dirt"), 2),
            ),
            Recipe::new(
                Farmer::Crafting,
                item!(m!("pumpkin")),
                item!(m!("pumpkin_seeds"), 4),
            ),
            Recipe::new(
                Fletcher::Crafting,
                item!(m!("rotten_flesh"), 4),
                item!(m!("leather"), 3),
            ),
            Recipe::new(
                Fletcher::Crafting,
                item!(m!("vine"), 2),
                item!(m!("moss_block")),
            ),
        ]);

        // Copper Conversions
        {
            pack.add_recipes(["", "_cut"].into_iter().flat_map(|c| {
                once(Recipe::new(
                    Mechanic::Crafting,
                    item!(if c == "" {
                        m!("copper_block")
                    } else {
                        m!("cut_copper")
                    }),
                    item!(m!(format!("exposed{c}_copper"))),
                ))
                .chain(
                    [("exposed", "weathered"), ("weathered", "oxidized")]
                        .into_iter()
                        .map(move |(f, t)| {
                            Recipe::new(
                                Mechanic::Crafting,
                                item!(m!(format!("{f}{c}_copper"))),
                                item!(m!(format!("{t}{c}_copper"))),
                            )
                        }),
                )
            }));
        }
    }
}
