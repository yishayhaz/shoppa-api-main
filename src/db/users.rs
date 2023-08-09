use crate::prelude::*;
use axum::async_trait;
use bson::{doc, oid::ObjectId, Document};
use chrono::{DateTime, Utc};
use mongodb::{
    options::{AggregateOptions, FindOneAndUpdateOptions, FindOneOptions, UpdateOptions},
    results::UpdateResult,
};
use serde::{Deserialize, Serialize};
use shoppa_core::db::{
    aggregations,
    models::{
        Address, CartItem, DBModel, EmbeddedDocument, FileTypes, ItemVariants, Order, Product,
        ProductItemStatus, ProductStatus, Store, User, UserStatus, Variants,
    },
    populate::UsersPopulate,
    DBConection,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserAsGetMe {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub email: Option<String>,
    pub name: Option<String>,
    pub phone_number: Option<String>,
    pub status: UserStatus,
    pub created_at: DateTime<Utc>,
    pub last_login: Option<DateTime<Utc>>,
    pub total_cart_items: u32,
}

#[async_trait]
pub trait UserFunctions {
    async fn get_user_by_email(
        &self,
        email: &str,
        options: Option<FindOneOptions>,
        populate: Option<UsersPopulate>,
    ) -> Result<Option<User>>;

    async fn update_user_password(
        &self,
        user_id: &ObjectId,
        new_password: &str,
        options: Option<FindOneAndUpdateOptions>,
    ) -> Result<Option<User>>;

    async fn get_user_by_id_and_not_deleted_or_banned(
        &self,
        user_id: &ObjectId,
        options: Option<FindOneOptions>,
        populate: Option<UsersPopulate>,
    ) -> Result<Option<User>>;

    async fn add_product_to_cart<T>(
        &self,
        user_id: &ObjectId,
        cart_item: T,
        options: Option<UpdateOptions>,
    ) -> Result<UpdateResult>
    where
        T: Into<CartItem> + Send + Sync;

    async fn remove_product_from_cart(
        &self,
        user_id: &ObjectId,
        product_id: &ObjectId,
        item_id: &ObjectId,
        options: Option<UpdateOptions>,
    ) -> Result<UpdateResult>;

    async fn edit_product_in_cart(
        &self,
        user_id: &ObjectId,
        product_id: &ObjectId,
        item_id: &ObjectId,
        quantity: u32,
        options: Option<UpdateOptions>,
    ) -> Result<UpdateResult>;

    async fn get_user_full_cart(
        &self,
        user_id: &ObjectId,
        options: Option<AggregateOptions>,
    ) -> Result<Vec<Document>>;

    async fn set_user_last_login(
        &self,
        user_id: &ObjectId,
        options: Option<FindOneAndUpdateOptions>,
    ) -> Result<Option<User>>;

    async fn add_user_address<T>(
        &self,
        user_id: &ObjectId,
        address: T,
        options: Option<UpdateOptions>,
    ) -> Result<UpdateResult>
    where
        T: Into<Address> + Send + Sync;

    async fn edit_user_address<T>(
        &self,
        user_id: &ObjectId,
        address_id: &ObjectId,
        address_update: T,
        options: Option<UpdateOptions>,
    ) -> Result<UpdateResult>
    where
        T: Into<Document> + Send + Sync;

    async fn delete_user_address(
        &self,
        user_id: &ObjectId,
        address_id: &ObjectId,
        options: Option<UpdateOptions>,
    ) -> Result<UpdateResult>;

    async fn update_user_after_order(&self, user: &User, order: &Order) -> Result<UpdateResult>;
}

// #[async_trait]
// pub trait UserAdminFunctions {}

#[async_trait]
impl UserFunctions for DBConection {
    async fn get_user_by_email(
        &self,
        email: &str,
        options: Option<FindOneOptions>,
        populate: Option<UsersPopulate>,
    ) -> Result<Option<User>> {
        let filters = doc! { User::fields().email: email, User::fields().status: {
            "$nin": [UserStatus::Deleted, UserStatus::Banned]
        } };

        self.get_user(filters, options, populate, None).await
    }

    async fn update_user_password(
        &self,
        user_id: &ObjectId,
        new_password: &str,
        options: Option<FindOneAndUpdateOptions>,
    ) -> Result<Option<User>> {
        let update = doc! { "$set": { User::fields().password: new_password } };

        self.find_and_update_user_by_id(user_id, update, options, None)
            .await
    }

    async fn get_user_by_id_and_not_deleted_or_banned(
        &self,
        user_id: &ObjectId,
        options: Option<FindOneOptions>,
        populate: Option<UsersPopulate>,
    ) -> Result<Option<User>> {
        let filters = doc! { User::fields().id: user_id, User::fields().status: {
            "$nin": [UserStatus::Deleted, UserStatus::Banned]
        } };

        self.get_user(filters, options, populate, None).await
    }

    async fn add_product_to_cart<T: Into<CartItem>>(
        &self,
        user_id: &ObjectId,
        cart_item: T,
        options: Option<UpdateOptions>,
    ) -> Result<UpdateResult>
    where
        T: Into<CartItem> + Send + Sync,
    {
        let cart_item: CartItem = cart_item.into();

        let filters = doc! {
            User::fields().id: user_id,
            User::fields().status: {
                "$nin": [UserStatus::Deleted, UserStatus::Banned]
            },
            // Check if the product is already in the cart
            // If it is, then we don't need to add it again
            User::fields().cart(true).items: {
                "$not": {
                    "$elemMatch": {
                        User::fields().cart(false).items(false).product: cart_item.product_id(),
                        User::fields().cart(false).items(false).item_id: &cart_item.item_id,
                    }
                }
            }
        };

        let update = doc! {
            "$push": {
                User::fields().cart(true).items: cart_item
            },
            "$currentDate": {
                User::fields().cart(true).last_updated: true
            }
        };

        self.update_user(filters, update, options, None).await
    }

    async fn remove_product_from_cart(
        &self,
        user_id: &ObjectId,
        product_id: &ObjectId,
        item_id: &ObjectId,
        options: Option<UpdateOptions>,
    ) -> Result<UpdateResult> {
        // by filtering only by this two fields,
        // if the matched count is 0, that means the user
        // got deleted or banned, so we can remove his access token
        let filters = doc! {
            User::fields().id: user_id,
            User::fields().status: {
                "$nin": [UserStatus::Deleted, UserStatus::Banned]
            },
        };

        let update = doc! {
            "$pull": {
                User::fields().cart(true).items: {
                    User::fields().cart(false).items(false).product: product_id,
                    User::fields().cart(false).items(false).item_id: item_id,
                }
            },
            "$currentDate": {
                User::fields().cart(true).last_updated: true
            }
        };

        self.update_user(filters, update, options, None).await
    }

    async fn edit_product_in_cart(
        &self,
        user_id: &ObjectId,
        product_id: &ObjectId,
        item_id: &ObjectId,
        quantity: u32,
        options: Option<UpdateOptions>,
    ) -> Result<UpdateResult> {
        let filters = doc! {
            User::fields().id: user_id,
            User::fields().status: {
                "$nin": [UserStatus::Deleted, UserStatus::Banned]
            },
            User::fields().cart(true).items: {
                "$elemMatch": {
                    User::fields().cart(false).items(false).product: product_id,
                    User::fields().cart(false).items(false).item_id: item_id,
                }
            }
        };

        let update = doc! {
            "$set": {
                format!("{}.$.{}",
                    User::fields().cart(true).items,
                    User::fields().cart(false).items(false).quantity
                ): quantity
            },
            "$currentDate": {
                User::fields().cart(true).last_updated: true
            }
        };

        self.update_user(filters, update, options, None).await
    }

    async fn get_user_full_cart(
        &self,
        user_id: &ObjectId,
        options: Option<AggregateOptions>,
    ) -> Result<Vec<Document>> {
        let get_name_field = doc! {
            "$cond": {
                "if": {
                    "$eq": [
                        format!("${}", Product::fields().items(true).name),
                        None::<String>
                    ]
                },
                "then": format!("${}", Product::fields().name),
                "else": format!("${}", Product::fields().items(true).name)
            }
        };

        fn image_cond(asset_ref: bool) -> Document {
            let mut and = vec![
                doc! {
                    "$eq": [
                        format!("$$asset.{}", Product::fields().assets(false).file_type),
                        FileTypes::Image
                    ]
                },
                doc! {
                    "$eq": [
                        format!("$$asset.{}", Product::fields().assets(false).public),
                        true
                    ]
                },
                doc! {
                    "$eq": [
                        format!("$$asset.{}", Product::fields().assets(false).hidden),
                        false
                    ]
                },
            ];

            if asset_ref {
                and.push(doc! {
                    "$in": [
                        format!("$$asset.{}", Product::fields().assets(false).id),
                        format!("${}", Product::fields().items(true).assets_refs)
                    ]
                });
            }

            doc! {
                "$and": and
            }
        }

        let get_image_field = doc! {
            "$cond": {
                "if": {
                    "$eq": [
                        format!("${}", Product::fields().items(true).assets_refs),
                        []
                    ]
                },
                "then": {
                    "$first": aggregations::filter(
                        format!("${}", Product::fields().assets),
                        "asset",
                        image_cond(false)
                    )
                },
                "else": {
                    "$first": aggregations::filter(
                        format!("${}", Product::fields().assets),
                        "asset",
                        image_cond(true)
                    )
                }
            }
        };

        let pipeline = [
            aggregations::match_query(&doc! {
                User::fields().id: user_id,
                User::fields().status: {
                    "$nin": [UserStatus::Deleted, UserStatus::Banned]
                },
            }),
            aggregations::unwind(User::fields().cart(true).items, false),
            aggregations::replace_root(User::fields().cart(true).items),
            aggregations::lookup::<Product>(
                User::fields().cart(false).items(false).product,
                Product::fields().id,
                User::fields().cart(false).items(false).product,
                Some(vec![
                    aggregations::unwind(Product::fields().items, false),
                    aggregations::match_query(&doc! {
                        "$expr": {
                            "$eq": [
                                format!("${}", Product::fields().items(true).id),
                                "$$item_id"
                            ]
                        },
                        Product::fields().items(true).status: {
                            "$ne": ProductItemStatus::Deleted
                        },
                        Product::fields().status: {
                            "$ne": ProductStatus::Deleted
                        },
                    }),
                    aggregations::lookup_product_variants(None),
                    aggregations::project(
                        aggregations::ProjectIdOptions::Keep,
                        [Product::fields().store, Product::fields().status],
                        Some(doc! {
                            Product::fields().name: get_name_field,
                            // getting the variants in a nice format
                            Product::fields().variants:
                                aggregations::map(
                                    format!("${}", Product::fields().items(true).variants),
                                    "item_variant",
                                    // finding the current variant, and formatting it, there can be only one.
                                    aggregations::array_elem_at(
                                        aggregations::map(
                                            // getting the current variant (A TypeOf Variant DBModel)
                                            aggregations::filter(
                                                format!("${}", Product::fields().variants),
                                                "product_variant",
                                                doc!{
                                                    "$eq": [
                                                        format!("$$product_variant.{}", Variants::fields().id),
                                                        format!("$$item_variant.{}", ItemVariants::fields().variant_id)
                                                    ]
                                                }
                                            ),
                                            "current_variant",
                                            doc!{
                                                Variants::fields().id: format!("$$current_variant.{}", Variants::fields().id),
                                                Variants::fields().type_: format!("$$current_variant.{}", Variants::fields().type_),
                                                Variants::fields().name: format!("$$current_variant.{}", Variants::fields().name),
                                                // getting the currect variant value for the current item_variant
                                                "value": aggregations::array_elem_at(
                                                    aggregations::map(
                                                        // getting the value
                                                        aggregations::filter(
                                                            format!("$$current_variant.{}", Variants::fields().values),
                                                            "variant_value",
                                                            doc!{
                                                                "$eq": [
                                                                    "$$variant_value._id",
                                                                    format!("$$item_variant.{}", ItemVariants::fields().value_id)
                                                                ]
                                                            }
                                                        ),
                                                        "current_value",
                                                        "$$current_value"
                                                    ),
                                                    0
                                                )
                                            }
                                        ),
                                        0
                                    )
                                ),
                            "item_status": format!("${}", Product::fields().items(true).status),
                            "price": format!("${}", Product::fields().items(true).price),
                            "image": get_image_field,
                            "item_id": format!("${}", Product::fields().items(true).id),
                            Product::fields().items(false).in_storage: format!("${}", Product::fields().items(true).in_storage),

                        }),
                    ),
                ]),
                Some(doc! {
                    "item_id": format!("${}", User::fields().cart(false).items(false).item_id)
                }),
            ),
            aggregations::unwind(User::fields().cart(false).items(false).product, false),
            // grouping product in the cart by store
            aggregations::group(doc! {
                "_id": format!("${}.{}", User::fields().cart(false).items(false).product, Product::fields().store(true).id),
                "products": {
                    "$push": "$$ROOT"
                },
            }),
            aggregations::lookup::<Store>(
                "_id",
                Store::fields().id,
                "store",
                Some(vec![aggregations::project(
                    aggregations::ProjectIdOptions::Keep,
                    [
                        Store::fields().name,
                        Store::fields().min_order,
                        Store::fields().delivery_strategies,
                    ],
                    None,
                )]),
                None,
            ),
            aggregations::unwind("store", false),
            // just cleaning up the result
            // no need to do safe fields name here
            aggregations::unset(vec![
                "_id",
                "products.item_id",
                "products.product.store",
                "products.product.variants.value.created_at",
                "products.product.variants.value.updated_at",
                "products.product.image.created_at",
                "products.product.image.updated_at",
                "products.product.image.public",
                "products.product.image.hidden",
                "products.product.image._id",
            ]),
        ];

        self.aggregate_users(pipeline, options, None).await
    }

    async fn set_user_last_login(
        &self,
        user_id: &ObjectId,
        options: Option<FindOneAndUpdateOptions>,
    ) -> Result<Option<User>> {
        let update = doc! {
            "$currentDate": {
                User::fields().last_login: true
            }
        };

        self.find_and_update_user_by_id(user_id, update, options, None)
            .await
    }

    async fn add_user_address<T>(
        &self,
        user_id: &ObjectId,
        address: T,
        options: Option<UpdateOptions>,
    ) -> Result<UpdateResult>
    where
        T: Into<Address> + Send + Sync,
    {
        let address: Address = address.into();

        let filters = doc! {
            User::fields().id: user_id,
            User::fields().status: {
                "$nin": [UserStatus::Deleted, UserStatus::Banned]
            }
        };

        let update = doc! {
            "$push": {
                User::fields().addresses: address.into_bson()?
            },
            // "$currentDate": {
            //     User::fields().updated_at: true
            // }
        };

        self.update_user(filters, update, options, None).await
    }

    async fn edit_user_address<T>(
        &self,
        user_id: &ObjectId,
        address_id: &ObjectId,
        address_update: T,
        options: Option<UpdateOptions>,
    ) -> Result<UpdateResult>
    where
        T: Into<Document> + Send + Sync,
    {
        let filters = doc! {
            User::fields().id: user_id,
            User::fields().status: {
                "$nin": [UserStatus::Deleted, UserStatus::Banned]
            },
            User::fields().addresses(true).id: address_id
        };

        let address_update: Document = address_update.into();
        tracing::info!("address_update: {:?}", &address_update);
        let address_update: Document = address_update
            .into_iter()
            .map(|(mut key, value)| {
                key = format!("{}.$.{}", User::fields().addresses, key);
                (key, value)
            })
            .collect();
        tracing::info!("address_update: {:?}", &address_update);

        let update = doc! {
            "$set": address_update,
            "$currentDate": {
                format!("{}.$.{}", User::fields().addresses, User::fields().addresses(false).updated_at): true
            }
        };

        self.update_user(filters, update, options, None).await
    }

    async fn delete_user_address(
        &self,
        user_id: &ObjectId,
        address_id: &ObjectId,
        options: Option<UpdateOptions>,
    ) -> Result<UpdateResult> {
        let filters = doc! {
            User::fields().id: user_id,
            User::fields().status: {
                "$nin": [UserStatus::Deleted, UserStatus::Banned]
            }
        };

        let update = doc! {
            "$pull": {
                User::fields().addresses: {
                    Address::fields().id: address_id
                }
            },
            // "$currentDate": {
            //     User::fields().updated_at: true
            // }
        };

        self.update_user(filters, update, options, None).await
    }

    async fn update_user_after_order(&self, user: &User, order: &Order) -> Result<UpdateResult> {
        let mut set = doc! {
            User::fields().cart(true).items: [],
            User::fields().is_verified: true,
            User::fields().had_first_order: true,
        };

        if user.phone_number.is_none() {
            set.insert(User::fields().phone_number, order.info.phone_number.clone());
        }

        let mut update = doc! {
            "$set": set,
        };

        if user.phone_number.is_none() {
            update.insert(
                "$currentDate",
                doc! {
                    User::fields().updated_at: true,
                },
            );
        }

        self.update_user_by_id(&user.id().unwrap(), update, None, None)
            .await
    }
}

impl From<User> for UserAsGetMe {
    fn from(user: User) -> Self {
        Self {
            id: user.id().unwrap().clone(),
            created_at: user.created_at().clone(),
            email: user.email,
            name: user.name,
            phone_number: user.phone_number,
            status: user.status,
            last_login: user.last_login,
            total_cart_items: user.cart.items.len() as u32,
        }
    }
}
