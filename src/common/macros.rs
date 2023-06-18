#[macro_export]
macro_rules! router {
    () => {
        pub(crate) fn router() -> Router {
            Router::new()
                .route("/", post(create))
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
            <$type>::by_id(db::db_pool(), id)
                .await?
                .ok_or_else(|| crate::api::error::Error::NotFound)
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
            <$type_entity>::by_id(db::db_pool(), id)
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
            <$type>::by_id(db::db_pool(), id)
                .await?
                .ok_or_else(|| crate::api::error::Error::NotFound)?
                .delete(db::db_pool())
                .await?
                .rows_affected()
                .expect(1)
        }
    };
}
