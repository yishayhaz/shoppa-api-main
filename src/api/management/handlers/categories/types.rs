use crate::prelude::{types::*, *};

#[derive(Deserialize)]
pub struct CreateCatgoryPayload {
    pub name: String,
    pub variants: Option<Vec<ObjectId>>,
    pub parent: Option<ObjectId>,
}

#[derive(Deserialize)]
pub struct EditCatetoryPayload {
    pub name: Option<String>,
    pub variants: Option<Vec<ObjectId>>,
}

impl Validate for EditCatetoryPayload {
    fn validate(&self) -> StdResult<(), ValidationErrors> {
        let mut errors = ValidationErrors::new();

        if !self.name.is_some() && !self.variants.is_some() {
            errors.add(
                "body",
                ValidationError::new("At least one field must be present"),
            );
        }

        if !errors.is_empty() {
            return Err(errors);
        }

        Ok(())
    }
}
