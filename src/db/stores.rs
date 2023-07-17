use crate::prelude::{types::*, *};
use axum::async_trait;
use bson::{doc, oid::ObjectId, Document};
use mongodb::options::{AggregateOptions, FindOneAndUpdateOptions};
use shoppa_core::{
    db::{
        aggregations::{self, ProjectIdOptions},
        models::{self, EmbeddedDocument, Store},
        DBConection, Pagination,
    },
    parser::FieldPatch,
};

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct DeliveryStrategiesUpdatePayload {
    pub default: FieldPatch<DefaultDeliveryUpdatePayload>,
    pub fast: FieldPatch<FastDeliveryUpdatePayload>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct DefaultDeliveryUpdatePayload {
    pub from_days: Option<u32>,
    pub to_days: Option<u32>,
    #[validate(range(min = 0.0))]
    pub price: Option<f64>,
    pub free_above: FieldPatch<f64>,
    pub comment: FieldPatch<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct FastDeliveryUpdatePayload {
    pub from_days: Option<u32>,
    pub to_days: Option<u32>,
    #[validate(range(min = 0.0))]
    pub price: Option<f64>,
    pub comment: FieldPatch<String>,
}

#[async_trait]
pub trait StoreFunctions {
    async fn get_random_stores_names(
        &self,
        options: Option<AggregateOptions>,
    ) -> Result<Vec<Document>>;
    async fn get_stores_names_for_autocomplete(
        &self,
        free_text: String,
        options: Option<AggregateOptions>,
    ) -> Result<Vec<Document>>;
    async fn get_many_stores_for_extarnel(
        &self,
        pagination: Option<Pagination>,
        free_text: Option<String>,
        options: Option<AggregateOptions>,
    ) -> Result<(Vec<Document>, u64)>;
    async fn get_store_for_extarnel(
        &self,
        store_id: &ObjectId,
        options: Option<AggregateOptions>,
    ) -> Result<Option<Document>>;
}

#[async_trait]
pub trait AdminStoreFunctions {
    async fn get_stores_for_admins(
        &self,
        pagination: Option<Pagination>,
        options: Option<AggregateOptions>,
    ) -> Result<(Vec<Document>, u64)>;

    async fn add_store_location(
        &self,
        store_id: &ObjectId,
        location: &models::StoreLocation,
        options: Option<FindOneAndUpdateOptions>,
    ) -> Result<Option<Store>>;

    async fn update_store_location(
        &self,
        store_id: &ObjectId,
        location_id: &ObjectId,
        city: &Option<String>,
        street: &Option<String>,
        street_number: &Option<String>,
        free_text: FieldPatch<String>,
        phone: &Option<String>,
        options: Option<FindOneAndUpdateOptions>,
    ) -> Result<Option<Store>>;

    async fn delete_store_location(
        &self,
        store_id: &ObjectId,
        location_id: &ObjectId,
        options: Option<FindOneAndUpdateOptions>,
    ) -> Result<Option<Store>>;

    async fn update_store_base_data(
        &self,
        store_id: &ObjectId,
        store_logo: Option<Option<models::FileDocument>>,
        store_banner: Option<Option<models::FileDocument>>,
        name: Option<String>,
        description: Option<String>,
        slogan: FieldPatch<String>,
        contact_email: Option<String>,
        contact_phone: Option<String>,
        legal_id: Option<String>,
        business_type: Option<models::StoreBusinessType>,
        business_name: Option<String>,
        min_order: Option<u64>,
        delivery_strategies: Option<DeliveryStrategiesUpdatePayload>,
        option: Option<FindOneAndUpdateOptions>,
    ) -> Result<Option<Store>>;
}

#[async_trait]
pub trait StoreUserStoreFunctions {
    async fn add_store_location(
        &self,
        store_id: &ObjectId,
        location: &models::StoreLocation,
        options: Option<FindOneAndUpdateOptions>,
    ) -> Result<Option<Store>>;

    async fn update_store_location(
        &self,
        store_id: &ObjectId,
        location_id: &ObjectId,
        city: &Option<String>,
        street: &Option<String>,
        street_number: &Option<String>,
        free_text: FieldPatch<String>,
        phone: &Option<String>,
        options: Option<FindOneAndUpdateOptions>,
    ) -> Result<Option<Store>>;

    async fn delete_store_location(
        &self,
        store_id: &ObjectId,
        location_id: &ObjectId,
        options: Option<FindOneAndUpdateOptions>,
    ) -> Result<Option<Store>>;

    async fn update_store_base_data(
        &self,
        store_id: &ObjectId,
        store_logo: Option<models::FileDocument>,
        store_banner: Option<models::FileDocument>,
        description: Option<String>,
        slogan: FieldPatch<String>,
        contact_email: Option<String>,
        contact_phone: Option<String>,
        min_order: Option<u64>,
        delivery_strategies: Option<DeliveryStrategiesUpdatePayload>,
        option: Option<FindOneAndUpdateOptions>,
    ) -> Result<Option<Store>>;
}

#[async_trait]
impl StoreFunctions for DBConection {
    async fn get_random_stores_names(
        &self,
        options: Option<AggregateOptions>,
    ) -> Result<Vec<Document>> {
        let pipeline = [
            aggregations::sample(10),
            aggregations::project(ProjectIdOptions::Keep, [models::Store::fields().name], None),
        ];

        self.aggregate_stores(pipeline, options, None).await
    }

    async fn get_stores_names_for_autocomplete(
        &self,
        free_text: String,
        options: Option<AggregateOptions>,
    ) -> Result<Vec<Document>> {
        let pipeline = [
            aggregations::autocomplete_store_search(&free_text),
            aggregations::add_score_meta(),
            aggregations::sort_by_score(),
            aggregations::limit(10),
            aggregations::project(ProjectIdOptions::Keep, [models::Store::fields().name], None),
        ];

        self.aggregate_stores(pipeline, options, None).await
    }

    async fn get_many_stores_for_extarnel(
        &self,
        pagination: Option<Pagination>,
        free_text: Option<String>,
        options: Option<AggregateOptions>,
    ) -> Result<(Vec<Document>, u64)> {
        let pagination = pagination.unwrap_or_default();

        let pipeline = [
            aggregations::search_store(&free_text, &vec![], None),
            aggregations::add_score_meta(),
            aggregations::sort_by_score(),
            aggregations::skip(pagination.offset),
            aggregations::limit(pagination.amount),
            aggregations::project(
                ProjectIdOptions::Keep,
                [
                    Store::fields().name,
                    Store::fields().logo(true).path,
                    Store::fields().logo(true).file_name,
                    Store::fields().logo(true).mime_type,
                    Store::fields().logo(true).file_type,
                    Store::fields().banner(true).path,
                    Store::fields().banner(true).file_name,
                    Store::fields().banner(true).mime_type,
                    Store::fields().banner(true).file_type,
                    Store::fields().description,
                    Store::fields().slogan,
                    Store::fields().created_at,
                ],
                None,
            ),
        ];

        let stores = self.aggregate_stores(pipeline, options, None).await?;

        let count = stores.len();

        if !pagination.need_count(count) {
            return Ok((stores, pagination.calculate_total(count)));
        }

        Ok((stores, self.count_stores(None, None, None).await?))
    }

    async fn get_store_for_extarnel(
        &self,
        store_id: &ObjectId,
        options: Option<AggregateOptions>,
    ) -> Result<Option<Document>> {
        let filter = doc! {
            "_id": store_id,
        };

        let pipeline = [
            aggregations::match_query(&filter),
            aggregations::project(
                ProjectIdOptions::Keep,
                [
                    Store::fields().created_at,
                    Store::fields().updated_at,
                    Store::fields().name,
                    Store::fields().slogan,
                    Store::fields().description,
                    Store::fields().banner(true).path,
                    Store::fields().banner(true).file_name,
                    Store::fields().banner(true).mime_type,
                    Store::fields().banner(true).file_type,
                    Store::fields().logo(true).path,
                    Store::fields().logo(true).file_name,
                    Store::fields().logo(true).mime_type,
                    Store::fields().logo(true).file_type,
                    Store::fields().analytics(true).views,
                    Store::fields().analytics(true).rating(true).average,
                    Store::fields().locations,
                ],
                None,
            ),
        ];

        let store = self.aggregate_stores(pipeline, options, None).await?;

        let store = store.get(0).map(|s| s.to_owned());

        Ok(store)
    }
}

#[async_trait]
impl AdminStoreFunctions for DBConection {
    async fn get_stores_for_admins(
        &self,
        pagination: Option<Pagination>,
        options: Option<AggregateOptions>,
    ) -> Result<(Vec<Document>, u64)> {
        let pagination = pagination.unwrap_or_default();

        let pipeline = [
            aggregations::skip(pagination.offset),
            aggregations::limit(pagination.amount),
            aggregations::project(
                ProjectIdOptions::Keep,
                [
                    models::Store::fields().name,
                    models::Store::fields().created_at,
                    models::Store::fields().analytics,
                    models::Store::fields().contact,
                ],
                None,
            ),
        ];

        let stores = self.aggregate_stores(pipeline, options, None).await?;

        let count = stores.len();

        if !pagination.need_count(count) {
            return Ok((stores, pagination.calculate_total(count)));
        }

        Ok((stores, self.count_stores(None, None, None).await?))
    }

    async fn add_store_location(
        &self,
        store_id: &ObjectId,
        location: &models::StoreLocation,
        options: Option<FindOneAndUpdateOptions>,
    ) -> Result<Option<Store>> {
        let filters = doc! {
            "_id": store_id,
            // to make sure that the id is not in the store locations already
            Store::fields().locations(true).id: {
                "$ne": location.id()
            }
        };

        let update = doc! {
            "$push": {
                Store::fields().locations: location
            }
        };

        self.find_and_update_store(filters, update, options, None)
            .await
    }

    async fn update_store_location(
        &self,
        store_id: &ObjectId,
        location_id: &ObjectId,
        city: &Option<String>,
        street: &Option<String>,
        street_number: &Option<String>,
        free_text: FieldPatch<String>,
        phone: &Option<String>,
        options: Option<FindOneAndUpdateOptions>,
    ) -> Result<Option<Store>> {
        let filters = doc! {
            "_id": store_id,
            Store::fields().locations(true).id: location_id
        };

        let mut update = doc! {};

        let loca_key_dollar = format!("{}.{}", Store::fields().locations, "$");

        let locations_fields = Store::fields().locations(false);

        if let Some(city) = city {
            update.insert(format!("{loca_key_dollar}.{}", locations_fields.city), city);
        }

        if let Some(street) = street {
            update.insert(
                format!("{loca_key_dollar}.{}", locations_fields.street),
                street,
            );
        }

        if let Some(street_number) = street_number {
            update.insert(
                format!("{loca_key_dollar}.{}", locations_fields.street_number),
                street_number,
            );
        }

        if FieldPatch::Missing != free_text {
            update.insert(
                format!("{loca_key_dollar}.{}", locations_fields.free_text),
                free_text.into_option(),
            );
        }

        if let Some(phone) = phone {
            update.insert(
                format!("{loca_key_dollar}.{}", locations_fields.phone),
                phone,
            );
        }

        let update = doc! {
            "$set": update
        };

        self.find_and_update_store(filters, update, options, None)
            .await
    }

    async fn delete_store_location(
        &self,
        store_id: &ObjectId,
        location_id: &ObjectId,
        options: Option<FindOneAndUpdateOptions>,
    ) -> Result<Option<Store>> {
        let update = doc! {
            "$pull": {
                Store::fields().locations: {
                    "_id": location_id
                }
            }
        };

        self.find_and_update_store_by_id(store_id, update, options, None)
            .await
    }

    async fn update_store_base_data(
        &self,
        store_id: &ObjectId,
        store_logo: Option<Option<models::FileDocument>>,
        store_banner: Option<Option<models::FileDocument>>,
        name: Option<String>,
        description: Option<String>,
        slogan: FieldPatch<String>,
        contact_email: Option<String>,
        contact_phone: Option<String>,
        legal_id: Option<String>,
        business_type: Option<models::StoreBusinessType>,
        business_name: Option<String>,
        min_order: Option<u64>,
        delivery_strategies: Option<DeliveryStrategiesUpdatePayload>,
        option: Option<FindOneAndUpdateOptions>,
    ) -> Result<Option<Store>> {
        let mut update = doc! {};

        let delivery_strategies = delivery_strategies.unwrap_or_default();

        let mut pipeline_update = false;

        match delivery_strategies.fast {
            FieldPatch::Missing => {}
            FieldPatch::Null => {
                update.insert(
                    Store::fields().delivery_strategies(true).fast,
                    None::<String>,
                );
            }
            FieldPatch::Value(value) => {
                pipeline_update = true;
                update.insert(Store::fields().delivery_strategies(true).fast, value);
            }
        };

        match delivery_strategies.default {
            FieldPatch::Missing => {}
            FieldPatch::Null => {
                update.insert(
                    Store::fields().delivery_strategies(true).default,
                    None::<String>,
                );
            }
            FieldPatch::Value(value) => {
                pipeline_update = true;
                update.insert(Store::fields().delivery_strategies(true).default, value);
            }
        };

        if let Some(store_logo) = store_logo {
            if let Some(store_logo) = store_logo {
                update.insert(Store::fields().logo, store_logo.into_bson()?);
            } else {
                update.insert::<_, Option<&str>>(Store::fields().logo, None);
            }
        }

        if let Some(store_banner) = store_banner {
            if let Some(store_banner) = store_banner {
                update.insert(Store::fields().banner, store_banner.into_bson()?);
            } else {
                update.insert::<_, Option<&str>>(Store::fields().banner, None);
            }
        }

        if let Some(name) = name {
            update.insert(Store::fields().name, name);
        }

        if let Some(description) = description {
            update.insert(Store::fields().description, description);
        }

        if FieldPatch::Missing != slogan {
            update.insert(Store::fields().slogan, slogan.into_option());
        }

        if let Some(contact_email) = contact_email {
            update.insert(Store::fields().contact(true).email, contact_email);
        }

        if let Some(contact_phone) = contact_phone {
            update.insert(Store::fields().contact(true).phone, contact_phone);
        }

        if let Some(legal_id) = legal_id {
            update.insert(Store::fields().legal_information(true).legal_id, legal_id);
        }

        if let Some(business_type) = business_type {
            update.insert(
                Store::fields().legal_information(true).business_type,
                business_type,
            );
        }

        if let Some(business_name) = business_name {
            update.insert(Store::fields().legal_information(true).name, business_name);
        }

        if let Some(min_order) = min_order {
            update.insert(Store::fields().min_order, min_order as i64);
        }

        let update = doc! {
            "$set": update
        };

        if pipeline_update {
            return self
                .find_and_update_store_by_id(store_id, vec![update], option, None)
                .await;
        }

        self.find_and_update_store_by_id(store_id, update, option, None)
            .await
    }
}

#[async_trait]
impl StoreUserStoreFunctions for DBConection {
    async fn add_store_location(
        &self,
        store_id: &ObjectId,
        location: &models::StoreLocation,
        options: Option<FindOneAndUpdateOptions>,
    ) -> Result<Option<Store>> {
        let filters = doc! {
            "_id": store_id,
            // to make sure that the id is not in the store locations already
            Store::fields().locations(true).id: {
                "$ne": location.id()
            }
        };

        let update = doc! {
            "$push": {
                Store::fields().locations: location
            }
        };

        self.find_and_update_store(filters, update, options, None)
            .await
    }

    async fn update_store_location(
        &self,
        store_id: &ObjectId,
        location_id: &ObjectId,
        city: &Option<String>,
        street: &Option<String>,
        street_number: &Option<String>,
        free_text: FieldPatch<String>,
        phone: &Option<String>,
        options: Option<FindOneAndUpdateOptions>,
    ) -> Result<Option<Store>> {
        let filters = doc! {
            "_id": store_id,
            Store::fields().locations(true).id: location_id
        };

        let mut update = doc! {};

        let loca_key_dollar = format!("{}.{}", Store::fields().locations, "$");

        let locations_fields = Store::fields().locations(false);

        if let Some(city) = city {
            update.insert(format!("{loca_key_dollar}.{}", locations_fields.city), city);
        }

        if let Some(street) = street {
            update.insert(
                format!("{loca_key_dollar}.{}", locations_fields.street),
                street,
            );
        }

        if let Some(street_number) = street_number {
            update.insert(
                format!("{loca_key_dollar}.{}", locations_fields.street_number),
                street_number,
            );
        }

        if FieldPatch::Missing != free_text {
            update.insert(
                format!("{loca_key_dollar}.{}", locations_fields.free_text),
                free_text.into_option(),
            );
        }

        if let Some(phone) = phone {
            update.insert(
                format!("{loca_key_dollar}.{}", locations_fields.phone),
                phone,
            );
        }

        let update = doc! {
            "$set": update
        };

        self.find_and_update_store(filters, update, options, None)
            .await
    }

    async fn delete_store_location(
        &self,
        store_id: &ObjectId,
        location_id: &ObjectId,
        options: Option<FindOneAndUpdateOptions>,
    ) -> Result<Option<Store>> {
        let update = doc! {
            "$pull": {
                Store::fields().locations: {
                    "_id": location_id
                }
            }
        };

        self.find_and_update_store_by_id(store_id, update, options, None)
            .await
    }

    async fn update_store_base_data(
        &self,
        store_id: &ObjectId,
        store_logo: Option<models::FileDocument>,
        store_banner: Option<models::FileDocument>,
        description: Option<String>,
        slogan: FieldPatch<String>,
        contact_email: Option<String>,
        contact_phone: Option<String>,
        min_order: Option<u64>,
        delivery_strategies: Option<DeliveryStrategiesUpdatePayload>,
        option: Option<FindOneAndUpdateOptions>,
    ) -> Result<Option<Store>> {
        let mut update = doc! {};

        let delivery_strategies = delivery_strategies.unwrap_or_default();

        let mut pipeline_update = false;

        match delivery_strategies.fast {
            FieldPatch::Missing => {}
            FieldPatch::Null => {
                update.insert(
                    Store::fields().delivery_strategies(true).fast,
                    None::<String>,
                );
            }
            FieldPatch::Value(value) => {
                pipeline_update = true;
                update.insert(Store::fields().delivery_strategies(true).fast, value);
            }
        };

        match delivery_strategies.default {
            FieldPatch::Missing => {}
            FieldPatch::Null => {
                update.insert(
                    Store::fields().delivery_strategies(true).default,
                    None::<String>,
                );
            }
            FieldPatch::Value(value) => {
                pipeline_update = true;
                update.insert(Store::fields().delivery_strategies(true).default, value);
            }
        };

        if let Some(store_logo) = store_logo {
            update.insert(Store::fields().logo, store_logo.into_bson()?);
        }

        if let Some(store_banner) = store_banner {
            update.insert(Store::fields().banner, store_banner.into_bson()?);
        }

        if let Some(description) = description {
            update.insert(Store::fields().description, description);
        }

        if FieldPatch::Missing != slogan {
            update.insert(Store::fields().slogan, slogan.into_option());
        }

        if let Some(contact_email) = contact_email {
            update.insert(Store::fields().contact(true).email, contact_email);
        }

        if let Some(contact_phone) = contact_phone {
            update.insert(Store::fields().contact(true).phone, contact_phone);
        }

        if let Some(min_order) = min_order {
            update.insert(Store::fields().min_order, min_order as i64);
        }

        let update = doc! {
            "$set": update
        };

        if pipeline_update {
            return self
                .find_and_update_store_by_id(store_id, vec![update], option, None)
                .await;
        }

        self.find_and_update_store_by_id(store_id, update, option, None)
            .await
    }
}

impl Default for DeliveryStrategiesUpdatePayload {
    fn default() -> Self {
        Self {
            default: FieldPatch::Missing,
            fast: FieldPatch::Missing,
        }
    }
}

impl Into<bson::Bson> for DefaultDeliveryUpdatePayload {
    fn into(self) -> bson::Bson {
        let mut partiel_doc = doc! {};

        let full_doc = self.full_doc();

        if self.from_days.is_some() {
            partiel_doc.insert(
                Store::fields()
                    .delivery_strategies(true)
                    .default(true)
                    .from_days,
                self.from_days,
            );
        };

        if self.to_days.is_some() {
            partiel_doc.insert(
                Store::fields()
                    .delivery_strategies(true)
                    .default(true)
                    .to_days,
                self.to_days,
            );
        };

        if self.price.is_some() {
            partiel_doc.insert(
                Store::fields()
                    .delivery_strategies(true)
                    .default(true)
                    .price,
                self.price,
            );
        };

        match self.free_above {
            FieldPatch::Missing => {}
            FieldPatch::Null => {
                partiel_doc.insert(
                    Store::fields()
                        .delivery_strategies(true)
                        .default(true)
                        .free_above,
                    None::<String>,
                );
            }
            FieldPatch::Value(value) => {
                partiel_doc.insert(
                    Store::fields()
                        .delivery_strategies(true)
                        .default(true)
                        .free_above,
                    value,
                );
            }
        }

        match self.comment {
            FieldPatch::Missing => {}
            FieldPatch::Null => {
                partiel_doc.insert(
                    Store::fields()
                        .delivery_strategies(true)
                        .default(true)
                        .comment,
                    None::<String>,
                );
            }
            FieldPatch::Value(value) => {
                partiel_doc.insert(
                    Store::fields()
                        .delivery_strategies(true)
                        .default(true)
                        .comment,
                    value,
                );
            }
        };

        bson::Bson::Document(doc! {
            Store::fields()
                .delivery_strategies(true)
                .fast: {
                    "$cond": {
                        "if": {
                            "$eq": [
                                "$$this",
                                None::<String>
                            ]
                        },
                        "then": full_doc,
                        "else": {
                            "$mergeObjects": [
                                "$$this",
                                partiel_doc
                            ]
                        }
                    }
                }
        })
    }
}

impl Into<bson::Bson> for FastDeliveryUpdatePayload {
    fn into(self) -> bson::Bson {
        let mut partiel_doc = doc! {};

        let full_doc = self.full_doc();

        if self.from_days.is_some() {
            partiel_doc.insert(
                Store::fields()
                    .delivery_strategies(true)
                    .default(true)
                    .from_days,
                self.from_days,
            );
        };

        if self.to_days.is_some() {
            partiel_doc.insert(
                Store::fields()
                    .delivery_strategies(true)
                    .default(true)
                    .to_days,
                self.to_days,
            );
        };

        if self.price.is_some() {
            partiel_doc.insert(
                Store::fields()
                    .delivery_strategies(true)
                    .default(true)
                    .price,
                self.price,
            );
        };

        match self.comment {
            FieldPatch::Missing => {}
            FieldPatch::Null => {
                partiel_doc.insert(
                    Store::fields()
                        .delivery_strategies(true)
                        .default(true)
                        .comment,
                    None::<String>,
                );
            }
            FieldPatch::Value(value) => {
                partiel_doc.insert(
                    Store::fields()
                        .delivery_strategies(true)
                        .default(true)
                        .comment,
                    value,
                );
            }
        };

        bson::Bson::Document(doc! {
            Store::fields()
                .delivery_strategies(true)
                .fast: {
                    "$cond": {
                        "if": {
                            "$eq": [
                                "$$this",
                                None::<String>
                            ]
                        },
                        "then": full_doc,
                        "else": {
                            "$mergeObjects": [
                                "$$this",
                                partiel_doc
                            ]
                        }
                    }
                }
        })
    }
}

impl DefaultDeliveryUpdatePayload {
    fn can_convert_to_full_doc(&self) -> bool {
        // Only if all required fields are present
        self.from_days.is_some() && self.to_days.is_some() && self.price.is_some()
    }

    fn full_doc(&self) -> Option<Document> {
        if !self.can_convert_to_full_doc() {
            return None;
        }

        Some(doc! {
            Store::fields().delivery_strategies(true).default(false).price: self.price,
            Store::fields().delivery_strategies(true).default(false).from_days: self.from_days,
            Store::fields().delivery_strategies(true).default(false).to_days: self.to_days,
            Store::fields().delivery_strategies(true).default(false).free_above: self.free_above.as_ref().into_option(),
            Store::fields().delivery_strategies(true).default(false).comment: self.comment.as_ref().into_option()
        })
    }
}

impl FastDeliveryUpdatePayload {
    fn can_convert_to_full_doc(&self) -> bool {
        // Only if all required fields are present
        self.from_days.is_some() && self.to_days.is_some() && self.price.is_some()
    }

    fn full_doc(&self) -> Option<Document> {
        if !self.can_convert_to_full_doc() {
            return None;
        }

        Some(doc! {
            Store::fields().delivery_strategies(true).fast(false).price: self.price,
            Store::fields().delivery_strategies(true).fast(false).from_days: self.from_days,
            Store::fields().delivery_strategies(true).fast(false).to_days: self.to_days,
            Store::fields().delivery_strategies(true).fast(false).comment: self.comment.as_ref().into_option()
        })
    }
}
