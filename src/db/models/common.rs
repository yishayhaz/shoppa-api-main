use crate::prelude::{db_models::*, *};

// TODO add Des to the required trait for DBModel
pub trait DBModel: Serialize + Clone {
    fn get_collection_name() -> &'static str;
    fn get_indexes() -> Vec<IndexModel>;
    fn created_at(&self) -> DateTime<Utc>;
    fn updated_at(&self) -> DateTime<Utc>;
    fn id(&self) -> Result<&ObjectId>;
    fn update_id(&mut self, id: ObjectId) -> ();
}

pub trait EmbeddedDocument: Serialize + Clone {
    fn created_at(&self) -> DateTime<Utc>;
    fn updated_at(&self) -> DateTime<Utc>;
    fn id(&self) -> &ObjectId;
    fn into_bson(&self) -> Result<Bson>;
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(untagged)]
#[serde(bound = "")]
pub enum RefrenceField<P, N>
where
    P: Serialize + for<'a> Deserialize<'a> + Debug,
    N: Serialize + for<'a> Deserialize<'a> + Debug,
{
    Populated(P),
    NotPopulated(N),
}

macro_rules! db_model {
    ($Struct:ty) => {
        fn created_at(&self) -> DateTime<Utc> {
            self.created_at
        }

        fn updated_at(&self) -> DateTime<Utc> {
            self.updated_at
        }

        fn id(&self) -> Result<&ObjectId> {
            match &self.id {
                Some(id) => Ok(id),
                // TODO add error code here
                None => Err(Error::Static("TODO")),
            }
        }

        fn update_id(&mut self, id: ObjectId) -> () {
            match self.id {
                Some(_) => return (),
                None => (),
            }

            self.id = Some(id);
        }
    };
}

macro_rules! embedded_document {
    ($Struct:ty) => {
        fn created_at(&self) -> DateTime<Utc> {
            self.created_at
        }

        fn updated_at(&self) -> DateTime<Utc> {
            self.updated_at
        }

        fn id(&self) -> &ObjectId {
            &self.id
        }

        fn into_bson(&self) -> Result<Bson> {
            match bson::to_bson(&self) {
                Ok(b) => Ok(b),
                Err(_) => Err(Error::Static("TODO")),
            }
        }
    };
}

pub(crate) use {db_model, embedded_document};
