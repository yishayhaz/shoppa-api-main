pub struct VariantsFields {
    pub id: &'static str,
    pub created_at: &'static str,
    pub updated_at: &'static str,
    pub name: &'static str,
    pub values: &'static str,
}

pub const FIELDS: VariantsFields = VariantsFields{
    id: "_id",
    created_at: "created_at",
    updated_at: "updated_at",
    name: "name",
    values: "values"
};

pub struct VariantValueFields {
    pub id: &'static str,
    pub created_at: &'static str,
    pub updated_at: &'static str,
    pub label: &'static str,
    pub value: &'static str,
}

const VALUES_FIELDS: VariantValueFields = VariantValueFields{
    id: "_id",
    created_at: "created_at",
    updated_at: "updated_at",
    label: "label",
    value: "value"
};

impl VariantsFields {
    pub fn values(&self) -> &VariantValueFields {
        &VALUES_FIELDS
    }
}