use super::super::schame::{BsonType, MongoSchame};
use super::Store;

fn get_schame() {
    let builder = MongoSchame::builder();

    builder
        .bson_type(BsonType::Document)
        .add_defaults_to_schame()
        .add_property((
            Store::fields().name,
            MongoSchame::builder()
                .bson_type(BsonType::String)
                .min_length(3)
                .max_length(60)
                .build(),
        ))
        .add_property((
            Store::fields().description,
            MongoSchame::builder()
                .bson_type(BsonType::String)
                .min_length(20)
                .max_length(160)
                .build(),
        ))
        .add_property((
            Store::fields().slogan,
            MongoSchame::builder()
                .add_bson_type(BsonType::String)
                .add_bson_type(BsonType::Null)
                .min_length(8)
                .max_length(40)
                .build(),
        ));
}
