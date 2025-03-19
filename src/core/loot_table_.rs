use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Serialize, Deserialize, Hash)]
pub struct LootTable {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    functions: Option<Vec<Function>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pools: Option<Vec<Pool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    random_sequence: Option<String>,
}

impl LootTable {
    pub fn new() -> Self {
        Self {
            type_: None,
            functions: None,
            pools: None,
            random_sequence: None,
        }
    }
}

macro_rules! include_with {
    ($name:ident, $variable:ident, $type:ty, $conversion:expr) => {
        impl LootTable {
            pub fn $name(self, $variable: $type) -> Self {
                Self {
                    $variable: Some($conversion),
                    ..self
                }
            }
        }
    };
}

include_with!(with_type, type_, impl Into<String>, type_.into());
include_with!(with_functions, functions, Vec<Function>, functions);
include_with!(with_pools, pools, Vec<Pool>, pools);
include_with!(with_random_sequence, random_sequence, impl Into<String>, random_sequence.into());


#[derive(Clone, PartialEq, Serialize, Deserialize, Hash)]
pub struct Function {
    #[serde(rename = "function")]
    function_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    conditions: Option<Vec<Condition>>,
    #[serde(flatten)]
    details: FunctionDetails,
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
#[serde(tag = "function", content = "details")]
pub enum FunctionDetails {
    #[serde(rename = "minecraft:set_count")]
    SetCount {
        count: NumberProvider,
    },
    #[serde(rename = "minecraft:set_nbt")]
    SetNbt {
        tag: String,
    },
    #[serde(rename = "minecraft:set_attributes")]
    SetAttributes {
        modifiers: Vec<AttributeModifier>
    },
    #[serde(rename = "minecraft:looting_enchantment")]
    LootingEnchantment {
        count: NumberProvider,
        quality: NumberProvider
    },
    #[serde(rename = "minecraft:random_set_count")]
    RandomSetCount{
        counts: Vec<NumberProvider>,
    },
    #[serde(rename = "minecraft:exploration_map")]
    ExplorationMap{
        destination: Option<String>,
        decoration: Option<String>,
        zoom: i32,
        search_radius: i32,
        skip_existing_chunks: bool,
    },
    #[serde(other)]
    Generic,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Hash)]
pub struct AttributeModifier{
    name: String,
    attribute: String,
    operation: String,
    amount: NumberProvider,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
}

impl Function {
    pub fn new(function_type: impl Into<String>) -> Self {
        Self {
            function_type: function_type.into(),
            conditions: None,
            details: FunctionDetails::Generic, // Default to Generic, change with with_
        }
    }
}

macro_rules! include_with_function {
    ($name:ident, $variable:ident, $type:ty, $conversion:expr) => {
        impl Function {
            pub fn $name(self, $variable: $type) -> Self {
                Self {
                    $variable: Some($conversion),
                    ..self
                }
            }
        }
    };
}

include_with_function!(with_conditions, conditions, Vec<Condition>, conditions);



impl Function {
    pub fn with_set_count(mut self, count: NumberProvider) -> Self {
        self.details = FunctionDetails::SetCount { count };
        self
    }

    pub fn with_set_nbt(mut self, tag: String) -> Self {
        self.details = FunctionDetails::SetNbt { tag };
        self
    }

     pub fn with_set_attributes(mut self, modifiers: Vec<AttributeModifier>) -> Self {
        self.details = FunctionDetails::SetAttributes{modifiers};
        self
    }

    pub fn with_looting_enchantment(mut self, count: NumberProvider, quality: NumberProvider) -> Self{
        self.details = FunctionDetails::LootingEnchantment{count, quality};
        self
    }

    pub fn with_random_set_count(mut self, counts: Vec<NumberProvider>) -> Self{
        self.details = FunctionDetails::RandomSetCount{counts};
        self
    }

   pub fn with_exploration_map(mut self, destination: Option<String>, decoration: Option<String>, zoom: i32, search_radius: i32, skip_existing_chunks: bool) -> Self{
        self.details = FunctionDetails::ExplorationMap{
            destination: destination,
            decoration: decoration,
            zoom: zoom,
            search_radius: search_radius,
            skip_existing_chunks: skip_existing_chunks
        };
        self
    }
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Hash)]
pub struct Pool {
    #[serde(skip_serializing_if = "Option::is_none")]
    conditions: Option<Vec<Condition>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    functions: Option<Vec<Function>>,
    rolls: NumberProvider,
    #[serde(skip_serializing_if = "Option::is_none")]
    bonus_rolls: Option<NumberProvider>,
    entries: Vec<Entry>,
}

impl Pool {
    pub fn new(rolls: NumberProvider, entries: Vec<Entry>) -> Self {
        Self {
            conditions: None,
            functions: None,
            rolls,
            bonus_rolls: None,
            entries,
        }
    }
}

macro_rules! include_with_pool {
    ($name:ident, $variable:ident, $type:ty, $conversion:expr) => {
        impl Pool {
            pub fn $name(self, $variable: $type) -> Self {
                Self {
                    $variable: Some($conversion),
                    ..self
                }
            }
        }
    };
}

include_with_pool!(with_conditions, conditions, Vec<Condition>, conditions);
include_with_pool!(with_functions, functions, Vec<Function>, functions);
include_with_pool!(with_bonus_rolls, bonus_rolls, NumberProvider, bonus_rolls);

#[derive(Clone, PartialEq, Serialize, Deserialize, Hash)]
pub struct Condition {
    #[serde(rename = "condition")]
    condition_type: String,
     #[serde(flatten)]
    details: ConditionDetails,
}
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
#[serde(tag = "condition", content = "details")]
pub enum ConditionDetails{
    #[serde(rename = "minecraft:random_chance")]
    RandomChance{
        chance: f64
    },
     #[serde(rename = "minecraft:killed_by_player")]
    KilledByPlayer{

    },
    #[serde(other)]
    Generic,
}
impl Condition{
     pub fn new(condition_type: impl Into<String>) -> Self {
        Self {
            condition_type: condition_type.into(),
            details: ConditionDetails::Generic, // Default to Generic, change with with_
        }
    }
}

macro_rules! include_with_condition {
    ($name:ident, $variable:ident, $type:ty, $conversion:expr) => {
        impl Condition {
            pub fn $name(self, $variable: $type) -> Self {
                Self {
                    $variable: Some($conversion),
                    ..self
                }
            }
        }
    };
}

include_with_condition!(with_details, details, ConditionDetails, details);

impl Condition{
    pub fn with_random_chance(mut self, chance: f64) -> Self{
        self.details = ConditionDetails::RandomChance{chance};
        self
    }

    pub fn with_killed_by_player(mut self) -> Self{
        self.details = ConditionDetails::KilledByPlayer{};
        self
    }
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Hash)]
#[serde(untagged)]
pub enum NumberProvider {
    Constant(f64),
    Object(Box<NumberProviderObject>),
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Hash)]
pub struct NumberProviderObject {
    #[serde(rename = "type")]
    type_: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    min: Option<Box<NumberProvider>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max: Option<Box<NumberProvider>>,
    #[serde(skip_serializing_if = "Option::is_none")]
     value: Option<Box<NumberProvider>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    n: Option<Box<NumberProvider>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    p: Option<Box<NumberProvider>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    score: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scale: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    storage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<String>,
     #[serde(skip_serializing_if = "Option::is_none")]
    amount: Option<String>
    // Add other fields specific to number providers
    // ...
}
impl NumberProviderObject{
    pub fn new(type_: impl Into<String>) -> Self{
        Self{
            type_: type_.into(),
            min: None,
            max: None,
            value: None,
            n: None,
            p: None,
            target: None,
            score: None,
            scale: None,
            storage: None,
            path: None,
            amount: None
        }
    }
}

impl NumberProvider {
    pub fn uniform(min: NumberProvider, max: NumberProvider) -> Self {
        NumberProvider::Object(Box::new(NumberProviderObject::new("minecraft:uniform")
            .with_min(Box::new(min))
            .with_max(Box::new(max))))
    }
     pub fn constant(value: f64) -> Self {
        NumberProvider::Constant(value)
    }
    pub fn binomial(n: NumberProvider, p: NumberProvider) -> Self{
         NumberProvider::Object(Box::new(NumberProviderObject::new("minecraft:binomial")
            .with_n(Box::new(n))
            .with_p(Box::new(p))))
    }
     pub fn score(target: String, score_name: String) -> Self {
        NumberProvider::Object(Box::new(NumberProviderObject::new("minecraft:score")
            .with_target(target)
            .with_score(score_name)))
    }
    pub fn storage(storage_name: String, path_name: String) -> Self{
        NumberProvider::Object(Box::new(NumberProviderObject::new("minecraft:storage")
            .with_storage(storage_name)
            .with_path(path_name)))
    }
     pub fn enchantment_level(amount: String) -> Self{
         NumberProvider::Object(Box::new(NumberProviderObject::new("minecraft:enchantment_level")
            .with_amount(amount)))
    }
}

macro_rules! include_with_number_provider_object {
    ($name:ident, $variable:ident, $type:ty, $conversion:expr) => {
        impl NumberProviderObject {
            pub fn $name(self, $variable: $type) -> Self {
                Self {
                    $variable: Some($conversion),
                    ..self
                }
            }
        }
    };
}

include_with_number_provider_object!(with_min, min, Box<NumberProvider>, min);
include_with_number_provider_object!(with_max, max, Box<NumberProvider>, max);
include_with_number_provider_object!(with_value, value, Box<NumberProvider>, value);
include_with_number_provider_object!(with_n, n, Box<NumberProvider>, n);
include_with_number_provider_object!(with_p, p, Box<NumberProvider>, p);
include_with_number_provider_object!(with_target, target, String, target);
include_with_number_provider_object!(with_score, score, String, score);
include_with_number_provider_object!(with_scale, scale, Option<f64>, scale);
include_with_number_provider_object!(with_storage, storage, String, storage);
include_with_number_provider_object!(with_path, path, String, path);
include_with_number_provider_object!(with_amount, amount, String, amount);


#[derive(Clone, PartialEq, Serialize, Deserialize, Hash)]
pub struct Entry {
    #[serde(rename = "type")]
    entry_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    conditions: Option<Vec<Condition>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    functions: Option<Vec<Function>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    weight: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    quality: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    children: Option<Vec<Entry>>
}
impl Entry{
    pub fn new(entry_type:impl Into<String>) -> Self{
        Self{
            entry_type: entry_type.into(),
            name: None,
            value: None,
            conditions: None,
            functions: None,
            weight: None,
            quality: None,
            expand: None,
            children: None
        }
    }
}

macro_rules! include_with_entry {
    ($name:ident, $variable:ident, $type:ty, $conversion:expr) => {
        impl Entry {
            pub fn $name(self, $variable: $type) -> Self {
                Self {
                    $variable: Some($conversion),
                    ..self
                }
            }
        }
    };
}

include_with_entry!(with_name, name, impl Into<String>, name.into());
include_with_entry!(with_value, value, impl Into<String>, value.into());
include_with_entry!(with_conditions, conditions, Vec<Condition>, conditions);
include_with_entry!(with_functions, functions, Vec<Function>, functions);
include_with_entry!(with_weight, weight, i32, weight);
include_with_entry!(with_quality, quality, i32, quality);
include_with_entry!(with_expand, expand, bool, expand);
include_with_entry!(with_children, children, Vec<Entry>, children);
