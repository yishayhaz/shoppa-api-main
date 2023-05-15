use crate::db::models::common::{FILE_DOCUMENT_FIELDS, FileDocumentFields};

pub struct StoreFields {
    pub id: &'static str,
    pub created_at: &'static str,
    pub updated_at: &'static str,
    pub name: &'static str,
    pub description: &'static str,
    pub slogan: &'static str,
    pub contact: &'static str,
    pub locations: &'static str,
    pub banner: &'static str,
    pub logo: &'static str,
    pub analytics: &'static str,
}

pub struct StoreContactFields {
    pub email: &'static str,
    pub tel: &'static str,
}

pub struct StoreLocationFields {
    pub free_text: &'static str,
    pub city: &'static str,
    pub street: &'static str,
    pub street_number: &'static str,
    pub legal_id: &'static str,
    pub phone: &'static str,
}

pub struct StoreAnalyticsFields {
    pub views: &'static str,
    pub sales: &'static str,
    pub rating: &'static str,
    pub orders: &'static str,
}

pub struct StoreOrdersStatsFields {
    pub pending: &'static str,
    pub in_progress: &'static str,
    pub failed: &'static str,
    pub arrived: &'static str,
}


pub const FIELDS: StoreFields = StoreFields {
    id: "_id",
    created_at: "created_at",
    updated_at: "updated_at",
    name: "name",
    description: "description",
    banner: "banner",
    logo: "logo",
    slogan: "slogan",
    contact: "contact",
    locations: "locations",
    analytics: "analytics",
};


pub const STORE_CONTACT_FIELDS: StoreContactFields = StoreContactFields {
    email: "email",
    tel: "tel",
};


pub const STORE_LOCATION_FIELDS: StoreLocationFields = StoreLocationFields {
    free_text: "free_text",
    city: "city",
    street: "street",
    street_number: "street_number",
    legal_id: "legal_id",
    phone: "phone",
};

pub const STORE_ANALYTICS_FIELDS: StoreAnalyticsFields = StoreAnalyticsFields {
    views: "views",
    sales: "sales",
    rating: "rating",
    orders: "orders",
};

pub const STORE_ORDERS_STATS_FIELDS: StoreOrdersStatsFields = StoreOrdersStatsFields {
    pending: "pending",
    in_progress: "in_progress",
    failed: "failed",
    arrived: "arrived",
};



impl StoreFields {
    pub fn contact(&self, full_path: bool) -> &'static StoreContactFields {
        &STORE_CONTACT_FIELDS
    }
    pub fn locations(&self, full_path: bool) -> &'static StoreLocationFields {
        &STORE_LOCATION_FIELDS
    }
    pub fn analytics(&self, full_path: bool) -> &'static StoreAnalyticsFields {
        &STORE_ANALYTICS_FIELDS
    }
    pub fn banner(&self, full_path: bool) -> &'static FileDocumentFields {
        &FILE_DOCUMENT_FIELDS
    }
    pub fn logo(&self, full_path: bool) -> &'static FileDocumentFields {
        &FILE_DOCUMENT_FIELDS
    }

}




