use crate::{
    core::{crafters::Alchemist, datapack::DataPack, module::Module, recipe::Recipe},
    item, m, nbt,
};

const POTION_LINGERING_MIN_LEVEL: usize = 5;
const POTION_STRONG_MIN_LEVEL: usize = 2;
const POTION_LONG_MIN_LEVEL: usize = 4;
const POTION_SPLASH_MIN_LEVEL: usize = 3;

const POTIONS: [(&str, usize); 3] = [
    ("minecraft:potion", 1),
    ("minecraft:splash_potion", POTION_SPLASH_MIN_LEVEL),
    ("minecraft:lingering_potion", POTION_LINGERING_MIN_LEVEL),
];

pub struct BrewingModule;

impl DataPack {
    fn add_brew(
        &mut self,
        from_item: String,
        from_potion: String,
        ingredient: String,
        to_item: String,
        to_potion: String,
        min_level: usize,
    ) -> String {
        self.add_recipe(
            Recipe::new(
                Alchemist::Crafting,
                [
                    item!(ingredient),
                    item!(m!("blaze_powder")),
                    item!(nbt!(from_item, "Poition" = from_potion)),
                ],
                item!(nbt!(to_item, "Potion" = to_potion)),
            )
            .with_min_building_level(min_level),
        )
    }

    /// Adds several recipes for a certain potion recipe. First, makes the recipe for the regular,
    /// splash, and lingering potion types, then creates recipes to convert the "to_potion" into
    /// splash or lingering, and the tipped arrow
    fn brew(
        &mut self,
        from_potion: String,
        ingredient: String,
        to_potion: String,
        min_level: usize,
    ) {
        for (item, level) in POTIONS {
            self.add_brew(
                item.to_string(),
                from_potion.clone(),
                ingredient.clone(),
                item.to_string(),
                to_potion.clone(),
                min_level.max(level),
            );
        }
        // Convert the "to" potion into others
        self.add_brew(
            m!("potion"),
            to_potion.clone(),
            m!("gunpowder"),
            m!("splash_potion"),
            to_potion.clone(),
            min_level.max(POTION_SPLASH_MIN_LEVEL),
        );

        self.add_brew(
            m!("potion"),
            to_potion.clone(),
            m!("dragon_breath"),
            m!("lingering_potion"),
            to_potion.clone(),
            min_level.max(POTION_SPLASH_MIN_LEVEL),
        );

        self.add_recipe(
            Recipe::new(
                Alchemist::Crafting,
                [
                    item!(m!("arrow"), 8),
                    item!(nbt!(m!("lingering_potion"), "Poition" = &to_potion)),
                ],
                item!(nbt!(m!("tipped_arrow"), "Potion" = &to_potion), 8),
            )
            .with_min_building_level(5),
        );
    }

    fn brew_mc(
        &mut self,
        from_potion: &str,
        ingredient: &str,
        to_potion: &str,
        strong: bool,
        long: bool,
        min_level: usize,
    ) {
        self.brew(m!(from_potion), m!(ingredient), m!(to_potion), min_level);

        if strong {
            self.brew(
                m!(to_potion),
                m!("glowstone_dust"),
                m!(format!("strong_{to_potion}")),
                min_level.max(POTION_STRONG_MIN_LEVEL),
            );
        }

        if long {
            self.brew(
                m!(to_potion),
                m!("redstone"),
                m!(format!("long_{to_potion}")),
                min_level.max(POTION_LONG_MIN_LEVEL),
            );
        }
    }
}

impl Module for BrewingModule {
    fn apply(pack: &mut DataPack) {
        pack.add_recipe(Recipe::new(
            Alchemist::Crafting,
            [item!(m!("ender_pearl"), 16)],
            item!(m!("nether_star")),
        ));
        pack.add_recipe(Recipe::new(
            Alchemist::Crafting,
            [item!(m!("glass_bottle"))],
            item!(nbt!(m!("potion"), "Potion" = m!("water"))),
        ));
        pack.add_recipe(Recipe::new(
            Alchemist::Crafting,
            [item!(m!("lava_bucket")), item!(m!("glass_bottle"), 8)],
            item!(m!("dragon_breath"), 2),
        ));

        pack.add_recipe(Recipe::new(
            Alchemist::Crafting,
            [item!(m!("gold_nugget"), 8), item!(m!("melon_slice"))],
            item!(m!("glistering_melon_slice")),
        ));

        let fse = "fermented_spider_eye";
        pack.brew_mc("water", "nether_wart", "awkward", false, false, 1);
        pack.brew_mc("awkward", "sugar", "swiftness", true, true, 1);
        pack.brew_mc("awkward", "rabbit_foot", "leaping", true, true, 1);
        pack.brew_mc("swiftness", fse, "slowness", true, true, 2);
        pack.brew_mc(
            "long_swiftness",
            fse,
            "long_slowness",
            false,
            false,
            POTION_LONG_MIN_LEVEL,
        );
        pack.brew_mc("leaping", fse, "slowness", false, false, 2);
        pack.brew_mc(
            "long_leaping",
            fse,
            "long_slowness",
            false,
            false,
            POTION_LONG_MIN_LEVEL,
        );
        pack.brew_mc("awkward", "blaze_powder", "strength", true, true, 1);
        pack.brew_mc(
            "awkward",
            "glistering_melon_slice",
            "healing",
            true,
            false,
            1,
        );
        pack.brew_mc("healing", fse, "harming", true, false, 2);
        pack.brew_mc(
            "strong_healing",
            fse,
            "strong_harming",
            false,
            false,
            POTION_STRONG_MIN_LEVEL,
        );
        pack.brew_mc("awkward", "spider_eye", "poison", true, true, 1);
        pack.brew_mc("poison", fse, "harming", false, false, 2);
        pack.brew_mc(
            "strong_poison",
            fse,
            "strong_harming",
            false,
            false,
            POTION_STRONG_MIN_LEVEL,
        );
        pack.brew_mc("awkward", "ghast_tear", "regeneration", true, true, 1);
        pack.brew_mc("awkward", "magma_cream", "fire_resistance", false, true, 1);
        pack.brew_mc("awkward", "pufferfish", "water_breathing", false, true, 1);
        pack.brew_mc("awkward", "golden_carrot", "night_vision", false, true, 1);
        pack.brew_mc("night_vision", fse, "invisibility", false, true, 2);
        pack.brew_mc(
            "long_night_vision",
            fse,
            "long_invisibility",
            false,
            false,
            POTION_LONG_MIN_LEVEL,
        );
        pack.brew_mc("awkward", "turtle_shell", "turtle_master", true, true, 4);
        pack.brew_mc(
            "awkward",
            "phantom_membrane",
            "slow_falling",
            false,
            true,
            5,
        );
        pack.brew_mc("water", fse, "weakness", false, true, 1);
    }
}
