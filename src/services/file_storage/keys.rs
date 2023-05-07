use crate::helpers::random;
use bson::oid::ObjectId;
use strum_macros::{Display, EnumString};

#[derive(Debug, Clone, Copy, Display, EnumString)]
enum StorageFolders {
    #[strum(to_string = "products/{id}/images/")]
    ProductsImages,
    #[strum(to_string = "products/{id}/videos/")]
    ProductsVideos,
    #[strum(to_string = "stores/{id}/images/")]
    StoresImages,
}

pub fn generate_product_image_key(product_id: &ObjectId, file_type: &String) -> String {
    let folder = StorageFolders::ProductsImages.to_string();

    let folder = folder.replace("{id}", &product_id.to_string());

    let file_name = random::random_string(32);

    format!("{}{}.{}", folder, file_name, file_type)
}

pub fn generate_store_logo_key(store_id: &ObjectId, file_type: &String) -> String {
    let folder = StorageFolders::StoresImages.to_string();

    let folder = folder.replace("{id}", &store_id.to_string());

    let file_name = "logo";

    format!("{}{}.{}", folder, file_name, file_type)
}

pub fn generate_store_banner_key(store_id: &ObjectId, file_type: &String) -> String {
    let folder = StorageFolders::StoresImages.to_string();

    let folder = folder.replace("{id}", &store_id.to_string());

    let file_name = "banner";

    format!("{}{}.{}", folder, file_name, file_type)
}
