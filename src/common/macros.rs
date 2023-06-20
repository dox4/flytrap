#[macro_export]
macro_rules! router {
    () => {
        pub(crate) fn router() -> Router {
            Router::new()
                .route("/", get(retrieve_list).post(create))
                .route("/:id", get(retrieve).put(update).delete(delete))
        }
    };
}

#[macro_export]
macro_rules! create {
    ($type_arg:ty, $type_entity:ty) => {
        async fn create(Json(arg): Json<$type_arg>) -> Result<$type_entity> {
            let entity: $type_entity = arg.into();
            let id = entity.id.clone();
            entity
                .create(db::db_pool())
                .await
                .map_err(|e| {
                    tracing::error!("created failed: {}", e);
                    crate::api::error::Error::CreateFailed(id.clone().to_string())
                })?
                .rows_affected()
                .expect(1)?;
            <$type_entity>::by_id(db::db_pool(), id)
                .await?
                .ok_or_else(|| crate::api::error::Error::NotFound)
        }
    };
}

#[macro_export]
macro_rules! retrieve {
    ($type:ty) => {
        async fn retrieve(Path(id): Path<Uuid>) -> Result<Request> {
            tracing::info!("retrieving");
            <$type>::by_id(db::db_pool(), id.hyphenated())
                .await?
                .ok_or_else(|| crate::api::error::Error::NotFound)
        }
    };
}

#[macro_export]
macro_rules! retrieve_list {
    ($type_query:ty, $type_entity:ty) => {
        async fn retrieve_list(
            Query(query): Query<$type_query>,
        ) -> Result<FetchPaged<$type_entity>> {
            let page = query.page.map(|i| if i == 0 { 1 } else { i }).unwrap_or(1);
            let per_page = query.per_page.unwrap_or(10);
            let offset = (page - 1) * per_page;
            let mut builder = sql_builder::SqlBuilder::select_from(<$type_entity>::table_name());
            query.query_with(&mut builder);
            let count_sql = builder.clone().count("0").sql().unwrap();
            let count: i64 = sqlx::query(&count_sql)
                .fetch_one(db::db_pool())
                .await?
                .get(0);
            builder.offset(offset).limit(per_page);
            let data_sql = builder.sql().unwrap();
            let list = sqlx::query_as::<_, $type_entity>(&data_sql)
                .fetch_all(db::db_pool())
                .await?;
            Ok((count, list).into())
        }
    };
}

#[macro_export]
macro_rules! update {
    ($type_arg:ty, $type_entity:ty) => {
        async fn update(
            Path(id): Path<Uuid>,
            Json(request): Json<$type_arg>,
        ) -> Result<RowsAffected> {
            <$type_entity>::by_id(db::db_pool(), id.hyphenated())
                .await?
                .ok_or_else(|| crate::api::error::Error::NotFound)?
                .update_with(request)
                .update(db::db_pool())
                .await?
                .rows_affected()
                .expect(1)
        }
    };
}

#[macro_export]
macro_rules! delete {
    ($type:ty) => {
        async fn delete(Path(id): Path<Uuid>) -> Result<RowsAffected> {
            <$type>::by_id(db::db_pool(), id.hyphenated())
                .await?
                .ok_or_else(|| crate::api::error::Error::NotFound)?
                .delete(db::db_pool())
                .await?
                .rows_affected()
                .expect(1)
        }
    };
}
