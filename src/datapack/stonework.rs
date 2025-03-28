use crate::{
    core::{crafters::Stonemason, datapack::DataPack, module::Module, recipe::Recipe},
    item, m,
};

struct Stone {
    base: String,
    block: String,
    stairs: bool,
    wall: bool,
    slab: bool,
}

impl Stone {
    fn stones() -> Vec<Self> {
        let mut stones = Vec::new();
        // Macros Instead

        stones.push(Self {
            base: "stone".to_string(),
            block: m!("stone"),
            wall: false,
            slab: true,
            stairs: true,
        });

        stones.extend(
            ["cobblestone", "mossy_cobblestone", "cobbled_deepslate"].map(|base| Self {
                base: base.to_string(),
                block: m!(base),
                wall: true,
                slab: true,
                stairs: true,
            }),
        );

        stones.extend(
            ["granite", "diorite", "andesite"]
                .into_iter()
                .flat_map(|b| {
                    [
                        Self {
                            base: b.to_string(),
                            block: m!(b),
                            stairs: true,
                            wall: true,
                            slab: true,
                        },
                        Self {
                            base: format!("polished_{b}"),
                            block: m!(format!("polished_{b}")),
                            stairs: true,
                            slab: true,
                            wall: false,
                        },
                    ]
                }),
        );

        stones.extend(
            [
                "cut_copper",
                "exposed_cut_copper",
                "weathered_cut_copper",
                "oxidized_cut_copper",
                "waxed_copper_block",
                "waxed_exposed_copper",
                "waxed_weathered_copper",
                "waxed_oxidized_copper",
                "waxed_cut_copper",
                "waxed_exposed_cut_copper",
                "waxed_weathered_cut_copper",
                "waxed_oxidized_cut_copper",
            ]
            .map(|base| Self {
                base: base.to_string(),
                block: m!(base),
                wall: false,
                slab: true,
                stairs: true,
            }),
        );

        stones
    }
}

pub struct StoneModule;

const STONECUTTING_LEVEL: usize = 3;

impl Module for StoneModule {
    fn apply(pack: &mut DataPack) {
        pack.add_recipes(["andesite", "granite", "diorite"].map(|block| {
            Recipe::new(
                Stonemason::Crafting,
                item!(m!(block), 4),
                item!(m!(format!("polished_{block}")), 4),
            )
        }));

        pack.add_recipes(
            Stone::stones()
                .iter()
                .flat_map(|stone| {
                    [
                        stone.stairs.then(|| {
                            (
                                6,
                                4,
                                stone.block.clone(),
                                m!(format!("{}_stairs", stone.base)),
                            )
                        }),
                        stone.slab.then(|| {
                            (
                                3,
                                6,
                                stone.block.clone(),
                                m!(format!("{}_slab", stone.base)),
                            )
                        }),
                        stone.wall.then(|| {
                            (
                                6,
                                6,
                                stone.block.clone(),
                                m!(format!("{}_wall", stone.base)),
                            )
                        }),
                    ]
                })
                .flatten()
                .flat_map(|(in_count, out_count, block, output)| {
                    [
                        Recipe::new(
                            Stonemason::Crafting,
                            item!(block.clone(), in_count),
                            item!(output.clone(), out_count),
                        )
                        .with_max_building_level(
                            STONECUTTING_LEVEL.checked_sub(1).unwrap_or(1).max(1),
                        ),
                        Recipe::new(
                            Stonemason::Crafting,
                            item!(block),
                            item!(output, 1.max(out_count / in_count)),
                        )
                        .with_min_building_level(STONECUTTING_LEVEL),
                    ]
                }),
        );

        // pack.add_recipes(Stone::stones().flat_map(|stone| {
        //     [
        //         Recipe::new(
        //             Stonemason::Crafting,
        //             item!(stone.block.clone(), 3),
        //             item!(format!("{}{}_slab", &stone.namespace, &stone.base), 6),
        //         )
        //         Recipe::new(
        //             Stonemason::Crafting,
        //             item!(stone.block.clone()),
        //             item!(format!("{}{}_slab", &stone.namespace, &stone.base)),
        //         )
        //         .with_min_building_level(STONECUTTING_LEVEL),
        //     ]
        // }));
    }
}
