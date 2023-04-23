use crate::db::aggregations;
use bson::Document;

pub trait PopulateOptions {
    fn build_pipeline(&self) -> Vec<Document>;
}

pub enum FieldPopulate<T: PopulateOptions> {
    Field,
    Nested(T),
    None,
}

pub struct ProductsPopulate {
    pub store: bool,
    pub categories: FieldPopulate<CategoriesPopulate>,
    pub variants: bool,
}

pub struct CategoriesPopulate {
    // between 0 - 3.
    // 0 is no populate
    // 1 is only first level
    // 2 is for the childs
    // 3 is also for grandchilds
    pub allowed_variants: u8,
}

impl PopulateOptions for ProductsPopulate {
    fn build_pipeline(&self) -> Vec<Document> {
        let mut pipeline: Vec<Document> = vec![];

        if self.variants {
            pipeline.push(aggregations::lookup_product_variants(None));
        };
        if self.store {
            pipeline.extend(aggregations::lookup_product_shop(None))
        }
        match self.categories {
            _ => {}
        }
        // TODO
        pipeline
    }
}

impl PopulateOptions for CategoriesPopulate {
    fn build_pipeline(&self) -> Vec<Document> {
        // TODO
        vec![Document::new()]
    }
}
