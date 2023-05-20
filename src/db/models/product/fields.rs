pub struct ProductFields {
    pub id: &'static str,
    pub created_at: &'static str,
    pub updated_at: &'static str,
    pub brand: &'static str,
    pub name: &'static str, // Todo: min: 10, max: 80
    pub description: &'static str, // Todo: min: 20, max: 150
    pub keywords: &'static str,
    pub store: &'static str,
    pub categories: &'static str,
    pub variants: &'static str,
    pub images: &'static str,
    pub items: &'static str,
    pub analytics: &'static str,
}

pub struct StoreFields {
    pub name: &'static str,
    pub id: &'static str,
}

pub struct CategoriesFields {
    pub name: &'static str,
    pub id: &'static str,
}

pub struct ItemsFields {
    pub id: &'static str,
    pub created_at: &'static str,
    pub updated_at: &'static str,
    pub price: &'static str,
    pub in_storage: &'static str,
    pub variants: &'static str,
    pub name: &'static str,
    pub images_refs: &'static str,
}


impl ProductFields {
    pub fn categories(&self) -> &'static CategoriesFields{
        &CATEGORIES_FIELDS
    }
    pub fn store(&self) -> &'static StoreFields {
        &STORE_FIELDS
    }
    pub fn items(&self) -> &'static ItemsFields {
        &ITEMS_FIELDS
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
    items: "items",
    analytics: "analytics",
    images: "images",
};

const CATEGORIES_FIELDS: CategoriesFields = CategoriesFields {
    id: "_id",
    name: "name",
};

const STORE_FIELDS: StoreFields = StoreFields {
    id: "_id",
    name: "name",
};

const ITEMS_FIELDS: ItemsFields = ItemsFields{
    id: "_id",
    created_at: "created_at",
    updated_at: "updated_at",
    price: "price",
    in_storage: "in_storage",
    variants: "variants",
    name: "name",
    images_refs: "images_refs",
};
