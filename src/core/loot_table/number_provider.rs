use serde::Serialize;

use crate::core::util::hashable_float::HF;


#[derive(Serialize)]
#[serde(untagged)]
pub enum NumberProvider {
    Constant(HF),
    Uniform {
        min: Box<NumberProvider>,
        max: Box<NumberProvider>,
    },
    Binomial {
        n: Box<NumberProvider>,
        p: Box<NumberProvider>,
    },
    Score {
        target: String,
        score: String,
    },
    Storage {
        storage: String,
        path: String,
    },
     EnchantmentLevel {
        amount: String
    },
    LootTable {
        name: String,
    },
    Tag {
        name: String,
    },
}
