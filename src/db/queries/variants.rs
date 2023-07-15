pub async fn get_variants_by_ids(
    db: &DBExtension,
    variants_ids: &Vec<ObjectId>,
) -> Result<Vec<Document>> {
    let pipeline = [
        aggregations::match_query(&doc! {
            Variants::fields().id: {
                "$in": variants_ids
            }
        }),
        aggregations::project(
            ProjectIdOptions::Keep,
            vec![Variants::fields().name, Variants::fields().values, "type"],
            None,
        ),
    ];

    let cursor = db
        .variants
        .aggregate(pipeline, None)
        .await
        .map_err(|e| Error::DBError(("variants", e)))?;

    let variants = cursor.consume().await?;

    Ok(variants)
}

