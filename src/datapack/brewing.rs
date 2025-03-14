use std::{collections::HashSet, iter::once};

use crate::{
    core::{crafters::Alchemist, datapack::DataPack, module::Module, recipe::Recipe},
    item, m, nbt,
};

pub struct BrewingModule;

impl Module for BrewingModule {
    fn apply(pack: &mut DataPack) {
        misc(pack);
    }
}

impl DataPack {
    fn brew<FI, FP, I, TI, TP>(
        &mut self,
        from_item: FI,
        from_potion: FP,
        ingredient: I,
        to_item: TI,
        to_potion: TP,
        min_level: usize,
    ) -> String
    where
        String: From<FI> + From<FP> + From<I> + From<TI> + From<TP>,
    {
        self.add_recipe(Recipe::new(
            Alchemist::Crafting,
            [
                item!(ingredient),
                item!(m!("blaze_powder")),
                item!(nbt!(from_item, "Potion" = from_potion)),
            ],
            item!(nbt!(from_item, "Potion" = to_potion)),
        ))
    }
}

struct Brewing<'a> {
    potions: HashSet<String>,
    pack: &'a mut DataPack,
}

impl<'a> Brewing<'a> {
    fn brew<FP, I, TP>(
        &'a mut self,
        from_potion: FP,
        ingredient: I,
        to_potion: TP,
        min_level: usize,
    ) where
        String: From<FP> + From<I> + From<TP>,
    {
        let from_potion = String::from(from_potion);
        let to_potion = String::from(to_potion);
        let ingredient = String::from(ingredient);

        self.potions.insert(from_potion.clone());
        self.potions.insert(to_potion.clone());

        for item in [m!("potion"), m!("splash_potion"), m!("lingering_potion")] {
            self.pack.brew(
                item.clone(),
                from_potion.clone(),
                ingredient.clone(),
                item,
                to_potion.clone(),
                min_level,
            );
        }
    }
}

fn potions<'a>(pack: &'a mut DataPack) {
    let mut b = Brewing::<'a> {
        potions: HashSet::new(),
        pack,
    };

    b.brew(m!("water"), m!("nether_wart"), m!("awkward"), 1);
    b.brew(m!("awkward"), m!("sugar"), m!("swiftness"), 1);
}

fn misc(pack: &mut DataPack) {
    pack.add_recipe(Recipe::new(
        Alchemist::Crafting,
        once(item!(m!("ender_pearl"), 16)),
        item!(m!("nether_star")),
    ));

    pack.add_recipe(Recipe::new(
        Alchemist::Crafting,
        once(item!(m!("glass_bottle"))),
        item!(nbt!(m!("potion"), "Potion" = "water")),
    ));

    pack.add_recipe(Recipe::new(
        Alchemist::Crafting,
        [item!(m!("lava_bucket")), item!(m!("glass_bottle"), 8)],
        item!(m!("dragon_breath"), 2),
    ));
}
