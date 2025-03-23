use crate::{
    core::{crafters::Stonemason, datapack::DataPack, module::Module, recipe::Recipe},
    item, m,
};

struct Stone {
    base: String,
    namespace: String,
    block: String,
}

impl Stone {
    fn stones() -> impl Iterator<Item = Self> {
        let mut basic_stones = vec![
            "stone",
            "cobblestone",
            "mossy_cobblestone",
            "cobbled_deepslate",
        ];

        basic_stones.extend([
            "copper_block",
            "exposed_copper",
            "weathered_copper",
            "oxidized_copper",
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
        ]);

        let plural_stones = vec!["stone_brick"];

        let basic = basic_stones.into_iter().map(|base| Self {
            base: base.to_owned(),
            namespace: m!(""),
            block: m!(base),
        });

        let plural = plural_stones.into_iter().map(|base| Self {
            base: base.to_owned(),
            namespace: m!(""),
            block: m!(format!("{base}s")),
        });

        basic.chain(plural)
    }
}

pub struct StoneModule;

const STONECUTTING_LEVEL: usize = 3;

impl Module for StoneModule {
    fn apply(pack: &mut DataPack) {
        pack.add_recipes(Stone::stones().flat_map(|stone| {
            [
                Recipe::new(
                    Stonemason::Crafting,
                    item!(stone.block.clone(), 6),
                    item!(format!("{}{}_stairs", &stone.namespace, &stone.base), 4),
                )
                .with_max_building_level(STONECUTTING_LEVEL.checked_sub(1).unwrap_or(1).max(1)),
                Recipe::new(
                    Stonemason::Crafting,
                    item!(stone.block.clone()),
                    item!(format!("{}{}_stairs", &stone.namespace, &stone.base)),
                )
                .with_min_building_level(STONECUTTING_LEVEL),
                Recipe::new(
                    Stonemason::Crafting,
                    item!(stone.block.clone(), 3),
                    item!(format!("{}{}_slab", &stone.namespace, &stone.base), 6),
                )
                .with_max_building_level(STONECUTTING_LEVEL.checked_sub(1).unwrap_or(1).max(1)),
                Recipe::new(
                    Stonemason::Crafting,
                    item!(stone.block.clone()),
                    item!(format!("{}{}_slab", &stone.namespace, &stone.base)),
                )
                .with_min_building_level(STONECUTTING_LEVEL),
            ]
        }));
    }
}
