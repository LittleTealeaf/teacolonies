use crate::{
    core::{crafters::Netherworker, datapack::DataPack, module::Module, recipe::Recipe},
    item, m, mc,
};

pub struct NetherModule;

impl Module for NetherModule {
    fn apply(pack: &mut DataPack) {
        pack.add_recipes([
            Recipe::new(
                Netherworker::Custom,
                item!(m!("iron_pickaxe")),
                item!(m!("nettherrack"), 64),
            ),
            Recipe::new(
                Netherworker::Custom,
                [item!(m!("lava_bucket"), 4), item!(m!("water_bucket"))],
                item!(m!("obsidian"), 8),
            )
            .with_tool(mc!("pickaxe"))
            .with_additional_output(item!(m!("bucket"), 5)),
            Recipe::new(
                Netherworker::Custom,
                [item!(m!("lava_bucket")), item!(m!("deepslate"), 4)],
                item!(m!("basalt"), 4),
            )
            .with_additional_output(item!(m!("bucket"))),
            Recipe::new(
                Netherworker::Custom,
                [item!(m!("lava_bucket")), item!(m!("netherrack"), 4)],
                item!(m!("blaze_rod"), 16),
            )
            .with_additional_output(item!(m!("bucket"))),
        ]);

        pack.add_recipes(["crimson", "warped"].into_iter().flat_map(|variant| {
            [
                Recipe::new(
                    Netherworker::Custom,
                    [
                        item!(m!("netherrack")),
                        item!(m!(format!("{variant}_fungus"))),
                    ],
                    item!(m!(format!("{variant}_nylium"))),
                ),
                Recipe::new(
                    Netherworker::Custom,
                    item!(m!(format!("{variant}_nylium"))),
                    item!(m!(format!("{variant}_roots")), 4),
                )
                .with_tool(m!("shears")),
            ]
        }));

        pack.add_recipes(
            [
                "shroomlight",
                "pearlescent_froglight",
                "verdant_froglight",
                "ochre_froglight",
            ]
            .map(|output| {
                Recipe::new(
                    Netherworker::Custom,
                    [item!(m!("glowstone")), item!(m!("mycelium"))],
                    item!(m!(output)),
                )
            }),
        );
    }
}
