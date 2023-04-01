pub struct ProductFields {
    pub id: &'static str,
    pub created_at: &'static str,
    pub updated_at: &'static str,
    pub brand: &'static str,
    pub name: &'static str,
    pub description: &'static str,
    pub keywords: &'static str,
    pub store: &'static str,
    pub categories: &'static str,
    pub variants: &'static str,
}

pub struct StoreFields {
    pub name: &'static str,
    pub id: &'static str,
}

pub struct CategoriesFields {
    pub name: &'static str,
    pub id: &'static str,
}

impl ProductFields {
    pub fn categories(&self) -> &'static CategoriesFields{
        &CATEGORIES_FIELDS
    }
    pub fn store(&self) -> &'static StoreFields {
        &STORE_FIELDS
    }
}

pub const FIELDS: ProductFields = ProductFields {
    id: "_id",
    created_at: "created_at",
    updated_at: "updated_at",
    brand: "brand",
    name: "name",
    description: "description",
    keywords: "keywords",
    store: "store",
    categories: "categories",
    variants: "variants",
};

const CATEGORIES_FIELDS: CategoriesFields = CategoriesFields {
    id: "_id",
    name: "name",
};

const STORE_FIELDS: StoreFields = StoreFields {
    id: "_id",
    name: "name",
};
