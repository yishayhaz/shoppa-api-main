pub struct CategoriesFields {
    pub id: &'static str,
    pub created_at: &'static str,
    pub updated_at: &'static str,
    pub name: &'static str,
    pub categories: &'static str
}

pub struct InnerCategoriesFields {
    pub id: &'static str,
    pub created_at: &'static str,
    pub updated_at: &'static str,
    pub name: &'static str,
    pub categories: &'static str
}

pub struct InnerInnerCategoriesFields {
    pub id: &'static str,
    pub created_at: &'static str,
    pub updated_at: &'static str,
    pub name: &'static str,
}

pub const FIELDS: CategoriesFields = CategoriesFields{
    id: "_id",
    created_at: "created_at",
    updated_at: "updated_at",
    name: "name",
    categories: "categories"
};


