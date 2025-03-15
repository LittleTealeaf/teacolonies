use crate::core::crafters::Fletcher;
use crate::core::item::Item;
use crate::core::{crafters::Dyer, datapack::DataPack, module::Module, recipe::Recipe};
use crate::{dom, item, m, mc};

pub struct DyerModule;

const COLORS: [&str; 16] = [
    "white",
    "light_gray",
    "light_blue",
    "gray",
    "black",
    "brown",
    "red",
    "orange",
    "yellow",
    "lime",
    "green",
    "cyan",
    "blue",
    "purple",
    "magenta",
    "pink",
];

impl Module for DyerModule {
    fn apply(pack: &mut DataPack) {
        add_dye_recipes(pack);
        for color in COLORS {
            add_recipes(pack, color);
        }
    }
}

fn add_recipes(dp: &mut DataPack, color: &str) {
    let dye = format!("{color}_dye");

    dp.add_recipes(
        [("terracotta", 8), ("candle", 1), ("shulker_box", 1)].map(|(block, count)| {
            Recipe::new(
                Dyer::Crafting,
                [item!(m!(dye)), item!(m!(block), count)],
                item!(m!(format!("{color}_{block}")), count),
            )
        }),
    );

    if color != "white" {
        dp.add_recipes(
            [("wool", 4), ("bed", 1), ("carpet", 1)].map(|(block, count)| {
                Recipe::new(
                    Dyer::Crafting,
                    [item!(m!(dye)), item!(m!(block), count)],
                    item!(m!(format!("{color}_{block}")), count),
                )
            }),
        );
    }

    dp.add_recipes(["glass_pane", "glass"].map(|style| {
        Recipe::new(
            Dyer::Crafting,
            [item!(m!(dye)), item!(m!(style), 8)],
            item!(m!(format!("{color}_stained_{style}")), 8),
        )
    }));

    dp.add_recipe(Recipe::new(
        Dyer::Crafting,
        [item!(m!(dye)), item!(m!("brick"), 4)],
        item!(dom!(format!("{color}_brick_extra")), 4),
    ));

    // Fletcher Based Recipes

    dp.add_recipe(Recipe::new(
        Fletcher::Crafting,
        [item!(m!(format!("{color}_wool")))],
        item!(mc!("colony_banner")),
    ));

    dp.add_recipe(Recipe::new(
        Fletcher::Crafting,
        [item!(m!(format!("{color}_wool")), 6), item!(m!("stick"))],
        item!(m!(format!("{color}_banner"))),
    ));

    dp.add_recipe(Recipe::new(
        Fletcher::Crafting,
        [item!(m!(format!("{color}_carpet"))), item!(m!("string"))],
        item!(dom!(format!("{color}_floating_carpet"))),
    ));
}

fn add_dye_recipes(pack: &mut DataPack) {
    pack.add_recipes(
        [
            (vec![item!(m!("bone_meal"))], item!(m!("white_dye"))),
            (
                vec![item!(m!("lily_of_the_valley"))],
                item!(m!("white_dye")),
            ),
            (vec![item!(m!("oxeye_daisy"))], item!(m!("light_gray_dye"))),
            (vec![item!(m!("azure_bluet"))], item!(m!("light_gray_dye"))),
            (vec![item!(m!("white_tulip"))], item!(m!("light_gray_dye"))),
            (
                vec![item!(m!("black_dye")), item!(m!("white_dye"), 2)],
                item!(m!("light_gray_dye"), 3),
            ),
            (
                vec![item!(m!("gray_dye")), item!(m!("white_dye"))],
                item!(m!("light_gray_dye"), 2),
            ),
            (vec![item!(m!("blue_orchid"))], item!(m!("light_blue_dye"))),
            (
                vec![item!(m!("blue_dye")), item!(m!("white_dye"))],
                item!(m!("light_blue_dye")),
            ),
            (
                vec![item!(m!("white_dye")), item!(m!("black_dye"))],
                item!(m!("gray_dye"), 2),
            ),
            (vec![item!(m!("ink_sac"))], item!(m!("black_dye"))),
            (vec![item!(m!("wither_rose"))], item!(m!("black_dye"))),
            (vec![item!(m!("cocoa_beans"))], item!(m!("brown_dye"))),
            (
                vec![item!(m!("fire_coral_block"))],
                item!(m!("red_dye"), 16),
            ),
            (vec![item!(m!("red_tulip"))], item!(m!("red_dye"))),
            (vec![item!(m!("beetroot"))], item!(m!("red_dye"))),
            (vec![item!(m!("poppy"))], item!(m!("red_dye"))),
            (vec![item!(m!("rose_bush"), 2)], item!(m!("red_dye"), 2)),
            (vec![item!(m!("torchflower"))], item!(m!("orange_dye"))),
            (vec![item!(m!("orange_tulip"))], item!(m!("orange_dye"))),
            (
                vec![item!(m!("red_dye")), item!(m!("yellow_dye"))],
                item!(m!("orange_dye"), 2),
            ),
            (vec![item!(m!("dandelion"))], item!(m!("yellow_dye"))),
            (vec![item!(m!("sunflower"), 2)], item!(m!("yellow_dye"))),
            (
                vec![item!(m!("horn_coral_block"))],
                item!(m!("yellow_dye"), 16),
            ),
            (
                vec![item!(m!("green_dye")), item!(m!("white_dye"))],
                item!(m!("lime_dye"), 2),
            ),
            (vec![item!(m!("kelp"))], item!(m!("green_dye"))),
            (vec![item!(m!("pitcher_plant"), 2)], item!(m!("cyan_dye"))),
            (
                vec![item!(m!("blue_dye")), item!(m!("green_dye"))],
                item!(m!("cyan_dye"), 2),
            ),
            (vec![item!(m!("cornflower"))], item!(m!("blue_dye"))),
            (
                vec![item!(m!("tube_coral_block"))],
                item!(m!("blue_dye"), 16),
            ),
            (vec![item!(m!("lapis_lazuli"))], item!(m!("blue_dye"))),
            (
                vec![item!(m!("blue_dye")), item!(m!("red_dye"))],
                item!(m!("purple_dye"), 2),
            ),
            (
                vec![
                    item!(m!("blue_dye")),
                    item!(m!("red_dye"), 2),
                    item!(m!("white_dye")),
                ],
                item!(m!("magenta_dye"), 4),
            ),
            (vec![item!(m!("allium"))], item!(m!("magenta_dye"))),
            (
                vec![
                    item!(m!("blue_dye")),
                    item!(m!("red_dye")),
                    item!(m!("pink_dye")),
                ],
                item!(m!("magenta_dye"), 3),
            ),
            (vec![item!(m!("lilac"), 2)], item!(m!("magenta_dye"))),
            (
                vec![item!(m!("purple_dye")), item!(m!("pink_dye"))],
                item!(m!("magenta_dye"), 2),
            ),
            (
                vec![item!(m!("bubble_coral_block"))],
                item!(m!("magenta_dye"), 16),
            ),
            (
                vec![item!(m!("red_dye")), item!(m!("white_dye"))],
                item!(m!("pink_dye"), 2),
            ),
            (
                vec![item!(m!("brain_coral_block"))],
                item!(m!("pink_dye"), 16),
            ),
            (vec![item!(m!("pink_petals"))], item!(m!("pink_dye"))),
            (vec![item!(m!("pink_tulip"))], item!(m!("pink_dye"))),
            (vec![item!(m!("peony"))], item!(m!("pink_dye"), 2)),
        ]
        .map(|(input, output)| Recipe::new(Dyer::Crafting, input, output)),
    );
}
