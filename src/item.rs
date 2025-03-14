use serde::{Deserialize, Serialize};

use crate::serde_util::exclude_if_one;

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct Item {
    #[serde(rename = "item")]
    name: String,
    #[serde(skip_serializing_if = "exclude_if_one")]
    count: usize,
}

impl Item {
    pub fn new(name: impl Into<String>, count: usize) -> Self {
        Self {
            name: name.into(),
            count,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn count(&self) -> usize {
        self.count
    }
}

#[macro_export]
macro_rules! item {
    ($name:expr) => {
        item!($name, 1)
    };
    ($name:expr, $count:expr) => {
        teacolonies::item::Item::new($name, $count)
    };
}

#[macro_export]
macro_rules! m {
    ($name: expr) => {
        format!("minecraft:{}", $name)
    };
}

#[macro_export]
macro_rules! dom {
    ($name: expr) => {
        format!("domum_ornamentum:{}", $name)
    };
}

#[macro_export]
macro_rules! mc {
    ($name: expr) => {
        format!("minecolonies:{}", $name)
    };
}

/// Adds nbt attributes
#[macro_export]
macro_rules! nbt {
    ($name: expr, $($field: literal = $value: expr),+) => {
        format!("{}{}", $name, nbt_rust::NbtTag::from(vec![
                $((String::from($field), nbt_rust::NbtTag::from($value))),+
        ]))
    };
}
