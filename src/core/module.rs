use super::datapack::DataPack;

pub trait Module {
    fn apply(pack: &mut DataPack);
}

impl DataPack {
    pub fn apply<I: Module>(&mut self) {
        I::apply(self);
    }
}
