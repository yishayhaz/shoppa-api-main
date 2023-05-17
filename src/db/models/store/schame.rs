use bson::Document;

use super::fields;

// {
//     validator: {
//        $jsonSchema: {
//           bsonType: "object",
//           required: [ "username", "password" ],
//           properties: {
//              username: {
//                 bsonType: "string",
//                 description: "must be a string and is required"
//              },
//              password: {
//                 bsonType: "string",
//                 minLength: 8,
//                 description: "must be a string at least 8 characters long, and is required"
//              }
//           }
//        }
//     }
//  }

struct MongoSchame {
    bson_type: String,
    required: Vec<&'static str>,
    properties: Document,
}

impl MongoSchame {

}