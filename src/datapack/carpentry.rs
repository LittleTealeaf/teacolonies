use std::iter::once;

use crate::{
    core::{crafters::Sawmill, datapack::DataPack, module::Module, recipe::Recipe},
    item, m,
};

pub struct CarpentryModule;

#[derive(Debug)]
struct Tree {
    root: String,
    log: String,
    wood: Option<String>,
    stripped_log: String,
    stripped_wood: Option<String>,
    sapling: Option<String>,
    leaves: Option<String>,
    level: usize,
}

impl Tree {
    fn basic(root: &str) -> Self {
        Self {
            root: root.to_string(),
            log: m!(format!("{root}_log")),
            stripped_log: m!(format!("stripped_{root}_log")),
            sapling: Some(m!(format!("{root}_sapling"))),
            leaves: Some(m!(format!("{root}_leaves"))),
            wood: Some(m!(format!("{root}_wood"))),
            stripped_wood: Some(m!(format!("stripped_{root}_wood"))),
            level: 1,
        }
    }

    fn trees() -> impl Iterator<Item = Self> {
        [
            "cherry", "acacia", "birch", "cherry", "dark_oak", "jungle", "oak", "spruce",
        ]
        .map(Self::basic)
        .into_iter()
        .chain(["crimson", "warped"].map(|root| Self {
            root: root.to_owned(),
            log: m!(format!("{root}_stem")),
            stripped_log: m!(format!("stripped_{root}_stem")),
            wood: Some(m!(format!("{root}_hyphae"))),
            stripped_wood: Some(m!(format!("stripped_{root}_hyphae"))),
            leaves: Some(if root == "crimson" {
                m!("nether_wart_block")
            } else {
                m!("warped_wart_block")
            }),
            sapling: Some(m!(format!("{root}_fungus"))),
            level: 3,
        }))
        .chain(once(Self {
            root: "bamboo".to_owned(),
            log: m!("bamboo_block"),
            stripped_log: m!("stripped_bamboo_block"),
            wood: None,
            stripped_wood: None,
            leaves: None,
            sapling: None,
            level: 2,
        }))
    }
}

impl Module for CarpentryModule {
    fn apply(pack: &mut DataPack) {
        Tree::trees().for_each(|tree| {
            let Tree {
                root,
                log,
                stripped_log,
                wood,
                stripped_wood,
                sapling,
                leaves,
                level,
            } = tree;
            let planks = m!(format!("{root}_planks"));
            let slab = m!(format!("{root}_slab"));

            pack.add_recipes([
                Recipe::new(
                    Sawmill::Crafting,
                    [item!(log)],
                    item!(planks.clone(), if root == "bamboo" { 2 } else { 4 }),
                ),
                Recipe::new(
                    Sawmill::Crafting,
                    [item!(planks.clone(), 3)],
                    item!(slab.clone(), 6),
                ),
            ]);
        });

        // println!("{:?}", Tree::trees().collect::<Vec<_>>())
    }
}
