use serde::{Deserialize, Serialize};

use crate::item::Item;
use crate::m;
use crate::serde_util::{exclude_if_false, exclude_if_one};

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct Recipe {
    #[serde(rename = "type")]
    type_: String,
    crafter: String,
    inputs: Vec<Item>,
    result: String,
    #[serde(skip_serializing_if = "exclude_if_one")]
    count: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    loot_table: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    additional_output: Option<Item>,
    #[serde(skip_serializing_if = "Option::is_none")]
    alternative_output: Option<Vec<Item>>,
    intermediate: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    research_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    not_research_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_building_level: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_building_level: Option<usize>,
    #[serde(skip_serializing_if = "exclude_if_false")]
    must_exist: bool,
    #[serde(skip_serializing_if = "exclude_if_false")]
    show_tooltip: bool,
}

impl Recipe {
    pub fn new(
        crafter: impl Into<String>,
        inputs: impl IntoIterator<Item = Item>,
        output: Item,
    ) -> Self {
        Self {
            type_: String::from("recipe"),
            crafter: crafter.into(),
            inputs: inputs.into_iter().collect(),
            result: output.name().to_string(),
            count: output.count(),
            tool: None,
            loot_table: None,
            additional_output: None,
            intermediate: m!("air"),
            research_id: None,
            not_research_id: None,
            min_building_level: None,
            max_building_level: None,
            must_exist: false,
            show_tooltip: false,
            alternative_output: None,
        }
    }
}

macro_rules! include_with {
    ($name: ident, $variable: ident, $type: ty, $conversion: expr) => {
        impl Recipe {
            pub fn $name(self, $variable: $type) -> Self {
                Self {
                    $variable: $conversion,
                    ..self
                }
            }
        }
    };
}

include_with!(
    with_loot_table,
    loot_table,
    impl Into<String>,
    Some(loot_table.into())
);

include_with!(
    with_additional_output,
    additional_output,
    Item,
    Some(additional_output)
);

include_with!(with_tool, tool, impl Into<String>, Some(tool.into()));

include_with!(
    with_intermediate,
    intermediate,
    impl Into<String>,
    intermediate.into()
);

include_with!(
    with_research_id,
    research_id,
    impl Into<String>,
    Some(research_id.into())
);
include_with!(
    with_not_research_id,
    not_research_id,
    impl Into<String>,
    Some(not_research_id.into())
);
include_with!(
    with_min_building_level,
    min_building_level,
    usize,
    Some(min_building_level)
);
include_with!(
    with_max_building_level,
    max_building_level,
    usize,
    Some(max_building_level)
);

include_with!(with_must_exist, must_exist, bool, must_exist);
include_with!(with_show_tooltip, show_tooltip, bool, show_tooltip);
