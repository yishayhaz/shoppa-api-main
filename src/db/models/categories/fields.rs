pub struct CategoriesFields {
    pub id: &'static str,
    pub created_at: &'static str,
    pub updated_at: &'static str,
    pub name: &'static str,
    pub categories: &'static str,
    pub allowed_variants: &'static str,
}

pub struct InnerCategoriesFields {
    pub id: &'static str,
    pub created_at: &'static str,
    pub updated_at: &'static str,
    pub name: &'static str,
    pub categories: &'static str,
    pub allowed_variants: &'static str,
}

pub struct InnerInnerCategoriesFields {
    pub id: &'static str,
    pub created_at: &'static str,
    pub updated_at: &'static str,
    pub name: &'static str,
    pub allowed_variants: &'static str,
}

impl CategoriesFields {
    pub fn categories() ->&'static InnerCategoriesFields {
        &INNER_FIELDS
    }
}

impl InnerCategoriesFields {
    pub fn categories() ->&'static InnerInnerCategoriesFields {
        &INNER_INNER_FIELDS
    }
}


pub const FIELDS: CategoriesFields = CategoriesFields{
    id: "_id",
    created_at: "created_at",
    updated_at: "updated_at",
    name: "name",
    categories: "categories",
    allowed_variants: "allowed_variants",
};

const INNER_FIELDS: InnerCategoriesFields = InnerCategoriesFields{
    id: "_id",
    created_at: "created_at",
    updated_at: "updated_at",
    name: "name",
    categories: "categories",
    allowed_variants: "allowed_variants",
};

const INNER_INNER_FIELDS: InnerInnerCategoriesFields = InnerInnerCategoriesFields{
    id: "_id",
    created_at: "created_at",
    updated_at: "updated_at",
    name: "name",
    allowed_variants: "allowed_variants",
};
