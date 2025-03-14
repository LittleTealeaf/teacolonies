pub struct DataPack {
    format: usize,
    description: String,
}

impl DataPack {
    pub fn new(description: String) -> Self {
        Self {
            format: 18,
            description,
        }
    }
}
