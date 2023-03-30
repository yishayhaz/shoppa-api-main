pub struct ContactUsFields {
    pub id: &'static str,
    pub created_at: &'static str,
    pub updated_at: &'static str,
    pub email: &'static str,
    pub message: &'static str,
    pub reason: &'static str,
}

pub const FIELDS: ContactUsFields = ContactUsFields {
    id: "_id",
    created_at: "created_at",
    updated_at: "updated_at",
    email: "email",
    message: "message",
    reason: "reason",
};
