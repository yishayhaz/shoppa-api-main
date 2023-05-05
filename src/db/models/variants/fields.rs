pub struct VariantsFields {
    pub id: &'static str,
    pub created_at: &'static str,
    pub updated_at: &'static str,
    pub name: &'static str,
    pub values: &'static str,
}

pub const FIELDS: VariantsFields = VariantsFields {
    id: "_id",
    created_at: "created_at",
    updated_at: "updated_at",
    name: "name",
    values: "values",
};

pub struct VariantValueFields {
    pub id: &'static str,
    pub created_at: &'static str,
    pub updated_at: &'static str,
    pub label: &'static str,
    pub value: &'static str,
}

const VALUES_FIELDS_RELATIVE: VariantValueFields = VariantValueFields {
    id: "_id",
    created_at: "created_at",
    updated_at: "updated_at",
    label: "label",
    value: "value",
};

const VALUES_FIELDS: VariantValueFields = VariantValueFields {
    id: "values._id",
    created_at: "values.created_at",
    updated_at: "values.updated_at",
    label: "values.label",
    value: "values.value",
};

impl VariantsFields {
    pub fn values(&self, relative: bool) -> &VariantValueFields {
        if relative {
            &VALUES_FIELDS_RELATIVE
        } else {
            &VALUES_FIELDS
        }
    }
}
