use crate::db::models::common::{FileDocumentFields, FILE_DOCUMENT_FIELDS};

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
    pub legal_information: &'static str,
}

pub struct StoreContactFields {
    pub email: &'static str,
    pub phone: &'static str,
}

pub struct StoreLocationFields {
    pub id: &'static str,
    pub free_text: &'static str,
    pub city: &'static str,
    pub street: &'static str,
    pub street_number: &'static str,
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

pub struct StoreRatingFields {
    pub votes: &'static str,
    pub average: &'static str,
}

pub struct StoreLegalInformation {
    pub legal_id: &'static str,
    pub business_type: &'static str,
    pub name: &'static str,
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
    legal_information: "legal_information",
};

pub const STORE_CONTACT_FIELDS: StoreContactFields = StoreContactFields {
    email: "email",
    phone: "phone",
};

pub const STORE_CONTACT_FIELDS_FULL_PATH: StoreContactFields = StoreContactFields {
    email: "contact.email",
    phone: "contact.phone",
};

pub const STORE_LOCATION_FIELDS: StoreLocationFields = StoreLocationFields {
    id: "_id",
    free_text: "free_text",
    city: "city",
    street: "street",
    street_number: "street_number",
    phone: "phone",
};

pub const STORE_LOCATION_FIELDS_FULL_PATH: StoreLocationFields = StoreLocationFields {
    id: "locations._id",
    free_text: "locations.free_text",
    city: "locations.city",
    street: "locations.street",
    street_number: "locations.street_number",
    phone: "locations.phone",
};

pub const STORE_ANALYTICS_FIELDS: StoreAnalyticsFields = StoreAnalyticsFields {
    views: "views",
    sales: "sales",
    rating: "rating",
    orders: "orders",
};

pub const STORE_ANALYTICS_FIELDS_FULL_PATH: StoreAnalyticsFields = StoreAnalyticsFields {
    views: "analytics.views",
    sales: "analytics.sales",
    rating: "analytics.rating",
    orders: "analytics.orders",
};

pub const STORE_ORDERS_STATS_FIELDS: StoreOrdersStatsFields = StoreOrdersStatsFields {
    pending: "pending",
    in_progress: "in_progress",
    failed: "failed",
    arrived: "arrived",
};

pub const STORE_ORDERS_STATS_FIELDS_FULL_PATH: StoreOrdersStatsFields = StoreOrdersStatsFields {
    pending: "analytics.orders.pending",
    in_progress: "analytics.orders.in_progress",
    failed: "analytics.orders.failed",
    arrived: "analytics.orders.arrived",
};

pub const STORE_RATING_FIELDS: StoreRatingFields = StoreRatingFields {
    votes: "votes",
    average: "average",
};

pub const STORE_RATING_FIELDS_FULL_PATH: StoreRatingFields = StoreRatingFields {
    votes: "analytics.rating.votes",
    average: "analytics.rating.average",
};

pub const STORE_LEGAL_INFORMATION_FIELDS: StoreLegalInformation = StoreLegalInformation {
    legal_id: "legal_id",
    business_type: "business_type",
    name: "name",
};

pub const STORE_LEGAL_INFORMATION_FIELDS_FULL_PATH: StoreLegalInformation = StoreLegalInformation {
    legal_id: "legal_information.legal_id",
    business_type: "legal_information.business_type",
    name: "legal_information.name",
};

pub const STORE_LOGO_FULL_PATH: FileDocumentFields = FileDocumentFields {
    id: "logo._id",
    public: "logo.public",
    hidden: "logo.hidden",
    file_name: "logo.file_name",
    path: "logo.path",
    size: "logo.size",
    mime_type: "logo.mime_type",
    file_type: "logo.file_type",
    created_at: "logo.created_at",
    updated_at: "logo.updated_at",
};

pub const STORE_BANNER_FULL_PATH: FileDocumentFields = FileDocumentFields {
    id: "banner._id",
    public: "banner.public",
    hidden: "banner.hidden",
    file_name: "banner.file_name",
    path: "banner.path",
    size: "banner.size",
    mime_type: "banner.mime_type",
    file_type: "banner.file_type",
    created_at: "banner.created_at",
    updated_at: "banner.updated_at",
};

impl StoreFields {
    pub fn contact(&self, full_path: bool) -> &'static StoreContactFields {
        if full_path {
            &STORE_CONTACT_FIELDS_FULL_PATH
        } else {
            &STORE_CONTACT_FIELDS
        }
    }
    pub fn locations(&self, full_path: bool) -> &'static StoreLocationFields {
        if full_path {
            &STORE_LOCATION_FIELDS_FULL_PATH
        } else {
            &STORE_LOCATION_FIELDS
        }
    }
    pub fn analytics(&self, full_path: bool) -> &'static StoreAnalyticsFields {
        if full_path {
            &STORE_ANALYTICS_FIELDS_FULL_PATH
        } else {
            &STORE_ANALYTICS_FIELDS
        }
    }
    pub fn banner(&self, full_path: bool) -> &'static FileDocumentFields {
        if full_path {
            &STORE_BANNER_FULL_PATH
        } else {
            &FILE_DOCUMENT_FIELDS
        }
    }
    pub fn logo(&self, full_path: bool) -> &'static FileDocumentFields {
        if full_path {
            &STORE_LOGO_FULL_PATH
        } else {
            &FILE_DOCUMENT_FIELDS
        }
    }

    pub fn legal_information(&self, full_path: bool) -> &'static StoreLegalInformation {
        if full_path {
            &STORE_LEGAL_INFORMATION_FIELDS_FULL_PATH
        } else {
            &STORE_LEGAL_INFORMATION_FIELDS
        }
    }
}

impl StoreAnalyticsFields {
    pub fn orders(&self, full_path: bool) -> &'static StoreOrdersStatsFields {
        if full_path {
            &STORE_ORDERS_STATS_FIELDS_FULL_PATH
        } else {
            &STORE_ORDERS_STATS_FIELDS
        }
    }
    pub fn rating(&self, full_path: bool) -> &'static StoreRatingFields {
        if full_path {
            &STORE_RATING_FIELDS_FULL_PATH
        } else {
            &STORE_RATING_FIELDS
        }
    }
}
