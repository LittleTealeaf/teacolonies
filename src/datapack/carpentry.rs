use std::iter::once;

use crate::{
    core::{
        crafters::{Forester, Mechanic, Sawmill},
        datapack::DataPack,
        module::Module,
        recipe::Recipe,
    },
    item, m, mc,
};

pub struct CarpentryModule;

#[derive(Debug)]
struct Tree {
    /// The prefix of the name, ex: "minecraft:". : is included
    root: String,
    tree: String,
    log: String,
    wood: Option<String>,
    stripped_log: String,
    stripped_wood: Option<String>,
    sapling: Option<String>,
    leaves: Option<String>,
    level: usize,
}

impl Tree {
    fn initialize_basic(tree: &str) -> Self {
        Self {
            root: m!(""),
            tree: tree.to_owned(),
            log: m!(format!("{tree}_log")),
            stripped_log: m!(format!("stripped_{tree}_log")),
            sapling: Some(m!(format!("{tree}_sapling"))),
            leaves: Some(m!(format!("{tree}_leaves"))),
            wood: Some(m!(format!("{tree}_wood"))),
            stripped_wood: Some(m!(format!("stripped_{tree}_wood"))),
            level: 1,
        }
    }

    fn trees() -> impl Iterator<Item = Self> {
        [
            "cherry", "acacia", "birch", "cherry", "dark_oak", "jungle", "oak", "spruce",
        ]
        .map(Self::initialize_basic)
        .into_iter()
        .chain(["crimson", "warped"].map(|tree| Self {
            root: m!(""),
            tree: tree.to_owned(),
            log: m!(format!("{tree}_stem")),
            stripped_log: m!(format!("stripped_{tree}_stem")),
            wood: Some(m!(format!("{tree}_hyphae"))),
            stripped_wood: Some(m!(format!("stripped_{tree}_hyphae"))),
            leaves: Some(if tree == "crimson" {
                m!("nether_wart_block")
            } else {
                m!("warped_wart_block")
            }),
            sapling: Some(m!(format!("{tree}_fungus"))),
            level: 3,
        }))
        .chain([
            Self {
                root: m!(""),
                tree: "bamboo".to_owned(),
                log: m!("bamboo_block"),
                stripped_log: m!("stripped_bamboo_block"),
                wood: None,
                stripped_wood: None,
                leaves: None,
                sapling: None,
                level: 2,
            },
            Self {
                sapling: Some(m!("mangrove_propagule")),
                level: 2,
                ..Self::initialize_basic("mangrove")
            },
        ])
    }

    fn var(&self, var: &str) -> String {
        format!("{}{}_{}", self.root, self.tree, var)
    }

    fn planks(&self) -> String {
        self.var("planks")
    }

    fn slab(&self) -> String {
        self.var("slab")
    }

    fn log(&self) -> String {
        self.log.clone()
    }

    fn stripped_log(&self) -> String {
        self.stripped_log.clone()
    }

    fn wood(&self) -> Option<String> {
        self.wood.clone()
    }

    fn stripped_wood(&self) -> Option<String> {
        self.stripped_wood.clone()
    }
}

impl Module for CarpentryModule {
    fn apply(pack: &mut DataPack) {
        Tree::trees().for_each(|t| {
            let mut recipes = Vec::new();

            if let Some(sapling) = &t.sapling {
                recipes.push(Recipe::new(
                    Forester::Custom,
                    [item!(mc!("compost"))],
                    item!(sapling.clone(), 4),
                ));
            }

            // Sawmill Recipes
            recipes.extend([
                Recipe::new(
                    Sawmill::Crafting,
                    [item!(t.log.clone())],
                    item!(t.planks(), if t.tree == "bamboo" { 2 } else { 4 }),
                ),
                Recipe::new(
                    Sawmill::Crafting,
                    [item!(t.planks(), 6), item!(m!("stick"))],
                    item!(t.var("sign"), 3),
                ),
                Recipe::new(
                    Sawmill::Crafting,
                    [item!(m!("chain"), 2), item!(t.stripped_log(), 6)],
                    item!(t.var("hanging_sign"), 3),
                ),
                Recipe::new(
                    Sawmill::Crafting,
                    [item!(t.planks(), 4), item!(m!("stick"), 2)],
                    item!(t.var("fence"), 3),
                ),
                Recipe::new(
                    Sawmill::Crafting,
                    [item!(t.planks(), 2), item!(m!("stick"), 4)],
                    item!(t.var("fence_gate"), 3),
                ),
                Recipe::new(
                    Sawmill::Crafting,
                    [item!(t.planks(), 6), item!(t.slab(), 2)],
                    item!(m!("barrel")),
                ),
                Recipe::new(
                    Sawmill::Crafting,
                    [item!(t.planks(), 6), item!(m!("honeycomb"), 3)],
                    item!(m!("beehive")),
                ),
                Recipe::new(
                    Sawmill::Crafting,
                    [item!(t.planks(), 2), item!(m!("stone_slab"))],
                    item!(m!("grindstone")),
                ),
            ]);

            // Minecolonies Crafting
            recipes.extend([
                Recipe::new(
                    Sawmill::Crafting,
                    [item!(t.planks(), 3), item!(m!("stick"), 6)],
                    item!(mc!("blockbarreldeco_onside")),
                ),
                Recipe::new(
                    Sawmill::Crafting,
                    [item!(t.planks(), 3), item!(m!("stick"), 6)],
                    item!(mc!("blockbarreldeco_standing")),
                ),
                Recipe::new(
                    Sawmill::Crafting,
                    [item!(t.planks(), 6), item!(m!("iron_nugget"), 2)],
                    item!(mc!("blockminecoloniesrack")),
                ),
                Recipe::new(
                    Sawmill::Crafting,
                    [item!(t.planks(), 6), item!(m!("chest"), 2)],
                    item!(mc!("bookstash")),
                ),
                Recipe::new(
                    Sawmill::Crafting,
                    [
                        item!(t.planks(), 6),
                        item!(m!("iron_ingot")),
                        item!(m!("dirt")),
                    ],
                    item!(mc!("barrel_block")),
                ),
            ]);

            // Mechanic Crafting
            recipes.extend([
                Recipe::new(
                    Mechanic::Crafting,
                    [item!(t.planks())],
                    item!(t.var("button")),
                ),
                Recipe::new(
                    Mechanic::Crafting,
                    [item!(t.planks(), 2)],
                    item!(t.var("pressure_plate")),
                ),
            ]);

            // Planks-Only Recipes
            recipes.extend(
                [
                    (2, vec![item!(m!("stick"), 4)]),
                    (
                        3,
                        vec![
                            item!(t.slab(), 6),
                            item!(t.var("stairs"), 4),
                            item!(t.var("door"), 3),
                        ],
                    ),
                    (4, vec![item!(m!("crafting_table"))]),
                    (6, vec![item!(t.var("trapdoor"), 2)]),
                    (8, vec![item!(m!("chest"))]),
                ]
                .into_iter()
                .flat_map(|(p, values)| {
                    let planks = t.planks();
                    values.into_iter().map(move |item| {
                        Recipe::new(Sawmill::Crafting, [item!(planks.clone(), p)], item)
                    })
                }),
            );

            pack.add_recipes(
                recipes
                    .into_iter()
                    .map(|recipe| recipe.add_min_building_level(t.level)),
            );
        });
    }
}
