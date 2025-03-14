

pub fn exclude_if_one(count: &usize) -> bool {
    *count == 1
}

pub fn exclude_if_true(value: &bool) -> bool {
    *value
}

pub fn exclude_if_false(value: &bool) -> bool {
    !*value
}
