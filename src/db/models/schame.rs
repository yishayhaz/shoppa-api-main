use super::common::{FILE_DOCUMENT_FIELDS};
use bson::{Document, Bson};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MongoSchame {
    additional_properties: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    bson_type: Option<Vec<BsonType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<&'static str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enum_: Option<Vec<&'static str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maximum: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_items: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_length: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    minimum: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_items: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_length: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pattern: Option<&'static str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    properties: Option<Document>,
    #[serde(skip_serializing_if = "Option::is_none")]
    required: Option<Vec<&'static str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<&'static str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unique_items: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    items: Option<Box<MongoSchame>>,
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum BsonType {
    Double,
    String,
    Array,
    #[serde(rename = "object")]
    Document,
    #[serde(rename = "bool")]
    Boolean,
    Null,
    RegularExpression,
    JavaScriptCode,
    JavaScriptCodeWithScope,
    #[serde(rename = "int")]
    Int32,
    #[serde(rename = "long")]
    Int64,
    Timestamp,
    Binary,
    ObjectId,
    #[serde(rename = "date")]
    DateTime,
    Decimal128,
    MaxKey,
    MinKey,
}

pub struct MongoSchameBuilder {
    additional_properties: Option<bool>,
    bson_type: Vec<BsonType>,
    description: Option<&'static str>,
    enum_: Vec<&'static str>,
    maximum: Option<i64>,
    max_items: Option<usize>,
    max_length: Option<usize>,
    minimum: Option<i64>,
    min_items: Option<usize>,
    min_length: Option<usize>,
    pattern: Option<&'static str>,
    properties: Vec<(&'static str, MongoSchame)>,
    required: Vec<&'static str>,
    title: Option<&'static str>,
    unique_items: Option<bool>,
    items: Option<MongoSchame>,
}

impl MongoSchame {
    pub fn builder() -> MongoSchameBuilder {
        MongoSchameBuilder::new()
    }
}

impl MongoSchameBuilder {
    pub fn new() -> Self {
        MongoSchameBuilder {
            bson_type: Vec::new(),
            required: Vec::new(),
            properties: Vec::new(),
            additional_properties: None,
            description: None,
            enum_: Vec::new(),
            maximum: None,
            max_items: None,
            max_length: None,
            minimum: None,
            min_items: None,
            min_length: None,
            pattern: None,
            title: None,
            unique_items: None,
            items: None,
        }
    }

    pub fn add_id_to_schame(self) -> Self {
        self.add_property((
            "_id",
            MongoSchame::builder().bson_type(BsonType::ObjectId).build(),
        ))
    }

    pub fn add_created_at_to_schame(self) -> Self {
        self.add_property((
            "created_at",
            MongoSchame::builder().bson_type(BsonType::DateTime).build(),
        ))
    }

    pub fn add_updated_at_to_schame(self) -> Self {
        self.add_property((
            "updated_at",
            MongoSchame::builder().bson_type(BsonType::DateTime).build(),
        ))
    }

    pub fn add_defaults_to_schame(self) -> Self {
        self.add_id_to_schame()
            .add_created_at_to_schame()
            .add_updated_at_to_schame()
    }

    pub fn require_defaults(self) -> Self {
        self.add_required("_id")
            .add_required("created_at")
            .add_required("updated_at")
    }

    pub fn require_all_properties(mut self) -> Self {
        let mut all_properties = Vec::new();

        for (key, _) in &self.properties {
            all_properties.push(*key);
        }

        self.required = all_properties;

        self
    }

    pub fn additional_properties(mut self, additional_properties: bool) -> Self {
        self.additional_properties = Some(additional_properties);
        self
    }

    pub fn bson_type(mut self, bson_type: BsonType) -> Self {
        self.bson_type = vec![bson_type];
        self
    }

    pub fn add_bson_type(mut self, bson_type: BsonType) -> Self {
        self.bson_type.push(bson_type);
        self
    }

    pub fn add_many_bson_type(mut self, bson_type: Vec<BsonType>) -> Self {
        self.bson_type.extend(bson_type);
        self
    }

    pub fn description(mut self, description: &'static str) -> Self {
        self.description = Some(description);
        self
    }

    pub fn enum_(mut self, enum_: Vec<&'static str>) -> Self {
        self.enum_ = enum_;
        self
    }

    pub fn add_enum_value(mut self, enum_: &'static str) -> Self {
        self.enum_.push(enum_);
        self
    }

    pub fn add_many_enum_values(mut self, enum_: Vec<&'static str>) -> Self {
        self.enum_.extend(enum_);
        self
    }

    pub fn maximum(mut self, maximum: i64) -> Self {
        self.maximum = Some(maximum);
        self
    }

    pub fn max_items(mut self, max_items: usize) -> Self {
        self.max_items = Some(max_items);
        self
    }

    pub fn max_length(mut self, max_length: usize) -> Self {
        self.max_length = Some(max_length);
        self
    }

    pub fn minimum(mut self, minimum: i64) -> Self {
        self.minimum = Some(minimum);
        self
    }

    pub fn min_items(mut self, min_items: usize) -> Self {
        self.min_items = Some(min_items);
        self
    }

    pub fn min_length(mut self, min_length: usize) -> Self {
        self.min_length = Some(min_length);
        self
    }

    pub fn pattern(mut self, pattern: &'static str) -> Self {
        self.pattern = Some(pattern);
        self
    }

    pub fn properties(mut self, properties: Vec<(&'static str, MongoSchame)>) -> Self {
        self.properties = properties;
        self
    }

    pub fn add_property(mut self, property: (&'static str, MongoSchame)) -> Self {
        self.properties.push(property);
        self
    }

    pub fn add_many_properties(mut self, properties: Vec<(&'static str, MongoSchame)>) -> Self {
        self.properties.extend(properties);
        self
    }

    pub fn add_required(mut self, required: &'static str) -> Self {
        self.required.push(required);
        self
    }

    pub fn add_many_required(mut self, required: Vec<&'static str>) -> Self {
        self.required.extend(required);
        self
    }

    pub fn title(mut self, title: &'static str) -> Self {
        self.title = Some(title);
        self
    }

    pub fn unique_items(mut self, unique_items: bool) -> Self {
        self.unique_items = Some(unique_items);
        self
    }

    pub fn file_properties(self, field: &'static str, allow_null: bool) -> Self {
        self.add_property((field, file_field_schema(allow_null)))
    }

    pub fn items(mut self, items: MongoSchame) -> Self {
        self.items = Some(items);
        self
    }

    pub fn build(self) -> MongoSchame {
        // add validation in the future

        MongoSchame {
            // default value in mongo is true
            additional_properties: self.additional_properties.unwrap_or(true),
            bson_type: if self.bson_type.is_empty() {
                None
            } else {
                Some(self.bson_type)
            },
            description: self.description,
            enum_: if self.enum_.is_empty() {
                None
            } else {
                Some(self.enum_)
            },
            maximum: self.maximum,
            max_items: self.max_items,
            max_length: self.max_length,
            minimum: self.minimum,
            min_items: self.min_items,
            min_length: self.min_length,
            pattern: self.pattern,
            properties: if self.properties.is_empty() {
                None
            } else {
                let mut properties = Document::new();

                for (key, value) in self.properties {
                    properties.insert(
                        key,
                        bson::to_bson(&value).expect("failed to convert to bson"),
                    );
                }
                Some(properties)
            },
            required: if self.required.is_empty() {
                None
            } else {
                Some(self.required)
            },
            title: self.title,
            unique_items: self.unique_items,
            items: self.items.map(|item| Box::new(item)),
        }
    }
}

fn file_field_schema(allow_null: bool) -> MongoSchame {
    let mut bson_types = vec![BsonType::Document];

    if allow_null {
        bson_types.push(BsonType::Null);
    }

    MongoSchame::builder()
        .add_many_bson_type(bson_types)
        .add_defaults_to_schame()
        .additional_properties(false)
        .add_property((
            FILE_DOCUMENT_FIELDS.public,
            MongoSchame::builder().bson_type(BsonType::Boolean).build(),
        ))
        .add_property((
            FILE_DOCUMENT_FIELDS.hidden,
            MongoSchame::builder().bson_type(BsonType::Boolean).build(),
        ))
        .add_property((
            FILE_DOCUMENT_FIELDS.file_name,
            MongoSchame::builder().bson_type(BsonType::String).build(),
        ))
        .add_property((
            FILE_DOCUMENT_FIELDS.path,
            MongoSchame::builder().bson_type(BsonType::String).build(),
        ))
        .add_property((
            FILE_DOCUMENT_FIELDS.size,
            MongoSchame::builder()
                .bson_type(BsonType::Int64)
                .minimum(0)
                .build(),
        ))
        .add_property((
            FILE_DOCUMENT_FIELDS.mime_type,
            MongoSchame::builder().bson_type(BsonType::String).build(),
        ))
        .add_property((
            FILE_DOCUMENT_FIELDS.file_type,
            MongoSchame::builder()
                .bson_type(BsonType::String)
                .enum_(vec!["image", "video", "audio", "document"])
                .build(),
        ))
        .require_all_properties()
        .build()
}

impl From<MongoSchame> for Bson {
    fn from(schame: MongoSchame) -> Self {
        bson::to_bson(&schame).expect("failed to convert schame to bson")
    }
}