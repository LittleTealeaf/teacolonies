use crate::{
    core::{datapack::DataPack, module::Module},
    m,
};

struct Stone {
    base: String,
    namespace: String,
    block: String,
}

impl Stone {
    fn simple(base: &str) -> Self {
        Self {
            base: base.to_string(),
            namespace: m!(""),
            block: m!(base),
        }
    }

    fn complex(base: String, block: String) -> Self {
        Self {
            base,
            namespace: m!(""),
            block,
        }
    }

    fn stones() -> impl IntoIterator<Item = Self> {
        [
            "stone",
            "cobblestone",
            "mossy_cobblestone",
            "cobbled_deepslate",
            "",
        ];
        todo!()

        // ["stone", "cobblestone", "mossy_cobblestone", "granite", "polished_granite"]
        //     .into_iter()
        //     .map(Self::simple)
    }
}

pub struct StoneModule;

impl Module for StoneModule {
    fn apply(pack: &mut DataPack) {}
}
