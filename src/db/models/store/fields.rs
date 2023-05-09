pub struct StoreFields {
    pub id: &'static str,
    pub created_at: &'static str,
    pub updated_at: &'static str,
    pub name: &'static str,
    pub description: &'static str,
    pub email: &'static str,
    pub location: &'static str,
    pub banner: &'static str,
    pub logo: &'static str,
}

pub const FIELDS: StoreFields = StoreFields {
    id: "_id",
    created_at: "created_at",
    updated_at: "updated_at",
    name: "name",
    description: "description",
    email: "email",
    location: "location",
    banner: "banner",
    logo: "logo",
};
