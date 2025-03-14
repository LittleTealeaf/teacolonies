use std::fmt::Display;

enum CraftingTypes {
    Crafting,
    Custom,
}

impl Display for CraftingTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Crafting => write!(f, "crafting"),
            Self::Custom => write!(f, "custom"),
        }
    }
}

macro_rules! crafter {
    ($id: ident, $name: expr, $($cid: ident),+) => {
        #[derive(PartialEq, Eq, Clone, Copy)]
        pub enum $id {
            $($cid),+
        }

        impl From<$id> for String {
            fn from(value: $id) -> Self {
                format!("{}_{}", $name, match value {
                    $($id::$cid => CraftingTypes::$cid),+
                    }
                )
            }
        }
    };
}

crafter!(Sawmill, "sawmill", Crafting);
crafter!(Mechanic, "mechanic", Crafting);
crafter!(Forester, "lumberjack", Custom);
crafter!(Blacksmith, "blacksmith", Crafting);
crafter!(Fletcher, "fletcher", Crafting);
crafter!(Builder, "builder", Crafting);
crafter!(Netherworker, "netherworker", Custom);
crafter!(Baker, "baker", Crafting);
crafter!(Farmer, "farmer", Crafting);
crafter!(Dyer, "dyer", Crafting);
crafter!(Sifter, "sifter", Custom);
crafter!(Stonemason, "stonemason", Crafting);
crafter!(Crusher, "cursher", Custom);
crafter!(Enchanter, "enchanter", Custom);
crafter!(Concrete, "concretemixer", Custom);
crafter!(Alchemist, "alchemist", Crafting);
crafter!(Chef, "chef", Crafting);
crafter!(Planter, "planter", Crafting);
