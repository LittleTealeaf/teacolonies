use std::{
    collections::HashMap,
    fs::File,
    hash::{DefaultHasher, Hash, Hasher},
    io::Write,
    path::Path,
};

use serde::Serialize;
use uuid::Uuid;
use zip::{write::FileOptions, ZipWriter};

use super::recipe::Recipe;

fn generate_uuid_from_hashable<T: Hash>(object: &T) -> Uuid {
    let mut hasher = DefaultHasher::new();
    object.hash(&mut hasher);
    let hash = hasher.finish();

    // Convert the 64-bit hash to a 128-bit UUID (version 5, name-based with SHA-1).
    // The namespace UUID is arbitrary; a constant is used here.
    let namespace_uuid = Uuid::from_u128(0x81878f82_3242_4562_9189_421042784262); // Arbitrary namespace UUID
    Uuid::new_v5(&namespace_uuid, &hash.to_le_bytes())
}

pub struct DataPack {
    name: String,
    recipes: HashMap<String, Recipe>,
}

impl DataPack {
    pub fn new(name: impl Into<String>) -> Self {
        let name = name.into();
        let recipes = HashMap::new();
        Self { name, recipes }
    }

    pub fn add_recipes(&mut self, recipes: impl IntoIterator<Item = Recipe>) {
        recipes.into_iter().for_each(|recipe| {
            self.add_recipe(recipe);
        });
    }

    pub fn add_recipe(&mut self, recipe: Recipe) -> String {
        let uuid = generate_uuid_from_hashable(&recipe).to_string();

        self.recipes.insert(uuid.clone(), recipe);

        uuid
    }

    pub fn save_to_zip(&self, path: &Path) -> std::io::Result<()> {
        #[derive(Serialize)]
        struct Meta {
            pack_format: usize,
            description: String,
        }

        let metadata = Meta {
            pack_format: 18,
            description: String::new(),
        };

        let file = File::create(path)?;
        let mut zip = ZipWriter::new(file);

        let options: FileOptions<()> =
            FileOptions::default().compression_method(zip::CompressionMethod::Deflated);

        zip.start_file("pack.mcmeta", options)?;
        serde_json::to_writer(&mut zip, &metadata)?;

        println!("Writing {} Recipes", &self.recipes.len());

        for (index, (id, recipe)) in self.recipes.iter().enumerate() {
            println!("Saving Recipe {index}: {id} for {}", recipe.result());
            let path = format!("data/{}/crafterrecipes/{}.json", self.name, id);
            zip.start_file(path, options)?;
            serde_json::to_writer(&mut zip, &recipe)?;
        }

        zip.finish()?;

        Ok(())
    }
}
