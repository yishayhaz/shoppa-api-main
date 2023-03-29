use super::{common::{db_model, DBModel}, prelude::*};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Store {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    created_at: DateTime<Utc>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    updated_at: DateTime<Utc>,

    pub name: String,
    pub email: String,
    pub location: StoreLocation,
    pub delivery_strategy: String
    // pub bank_details: ?
    //pub logo(s): ?
    //pub banner(s): ?


}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StoreLocation{
    pub free_text: String
}


impl DBModel for Store {
    fn get_collection_name() -> &'static str {
        "stores"
    }

    fn get_indexes() -> Vec<IndexModel> {
        let unique_index_options = IndexOptions::builder()
            .unique(true)
            .name(String::from("unique_store_name"))
            .build();

        let uniqe_index = IndexModel::builder()
            .keys(doc! {
                "name": 1
            })
            .options(unique_index_options)
            .build();

        vec![uniqe_index]
    }

    db_model!(Store);
}

// impl Store {
//     pub fn new(name: String) -> Self {
//         Self {
//             id: None,
//             created_at: Utc::now(),
//             updated_at: Utc::now(),
//             name,
//         }
//     }
// }
