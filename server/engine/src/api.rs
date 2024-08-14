use crate::{
    dto::{InsertableNewEdge, InsertableNewVertex, NewEdge, NewVertex},
    error::Error,
    model::{self, Edge, Vertex},
};
use diesel::{ExpressionMethods, QueryDsl, SelectableHelper};
use diesel_async::{AsyncPgConnection, RunQueryDsl};

async fn create_vertex(
    conn: &mut AsyncPgConnection,
    new_vertex: &NewVertex,
) -> Result<Vertex, Error> {
    use crate::schema::vertex::dsl::*;

    let new_vertex = InsertableNewVertex {
        name: new_vertex.name.clone(),
        type_: new_vertex.type_.clone(),
        created_by: new_vertex.created_by.clone(),
        updated_by: new_vertex.created_by.clone(),
    };

    let result = diesel::insert_into(vertex)
        .values(new_vertex)
        .returning(Vertex::as_returning())
        .get_result(conn)
        .await?;

    Ok(result)
}

async fn create_edge(conn: &mut AsyncPgConnection, new_edge: &NewEdge) -> Result<Edge, Error> {
    use crate::schema::edge::dsl::*;
    use crate::schema::vertex::dsl::*;
    use crate::schema::vertex::id as VertexId;

    let source_vertex_type = vertex
        .filter(VertexId.eq(new_edge.from_vertex_id))
        .select(type_)
        .first::<String>(conn)
        .await?;

    let target_vertex_type = vertex
        .filter(VertexId.eq(new_edge.to_vertex_id))
        .select(type_)
        .first::<String>(conn)
        .await?;

    let new_edge = InsertableNewEdge {
        from_vertex_id: new_edge.from_vertex_id,
        from_vertex_type: source_vertex_type,
        to_vertex_id: new_edge.to_vertex_id,
        to_vertex_type: target_vertex_type,
        label: new_edge.label.clone(),
        created_by: new_edge.created_by.clone(),
        updated_by: new_edge.created_by.clone(),
    };

    let result = diesel::insert_into(edge)
        .values(&new_edge)
        .returning(Edge::as_returning())
        .get_result(conn)
        .await?;

    Ok(result)
}

async fn get_vertex_by_id(conn: &mut AsyncPgConnection, vertext_id: i32) -> Result<Vertex, Error> {
    use crate::schema::vertex::dsl::*;

    if vertext_id < 1 {
        return Err(Error::Validation(validator::ValidationErrors::new()));
    }

    let result = vertex
        .filter(crate::schema::vertex::id.eq(vertext_id))
        .select(model::Vertex::as_select())
        .first::<Vertex>(conn)
        .await?;

    Ok(result)
}

async fn delete_vertex_by_id(
    conn: &mut AsyncPgConnection,
    vertext_id: i32,
) -> Result<usize, Error> {
    use crate::schema::vertex::dsl::*;

    if vertext_id < 1 {
        return Err(Error::Validation(validator::ValidationErrors::new()));
    }

    let result = diesel::delete(vertex.filter(crate::schema::vertex::id.eq(vertext_id)))
        .execute(conn)
        .await?;

    Ok(result)
}

#[cfg(test)]
mod tests {

    use crate::dto::NewVertex;
    use crate::schema::edge;
    use crate::schema::edge::dsl::*;
    use diesel::{ExpressionMethods, QueryDsl};
    use diesel_async::{AsyncConnection, AsyncPgConnection, RunQueryDsl};

    #[tokio::test]
    async fn test_create_vertex() {
        dotenvy::from_path(".env").ok();

        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        let new_vertex = NewVertex {
            name: "create_vertex".to_string(),
            type_: "create_vertex".to_string(),
            created_by: "test".to_string(),
        };

        let mut conn = AsyncPgConnection::establish(&database_url).await.unwrap();
        let result = crate::api::create_vertex(&mut conn, &new_vertex)
            .await
            .unwrap();
        assert!(result.id > 0);
        assert_eq!(result.name, "create_vertex");
        assert_eq!(result.type_, "create_vertex");
        assert_eq!(result.created_by, "test");
    }

    #[tokio::test]
    async fn test_create_edge() {
        dotenvy::from_path(".env").ok();

        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        let source_vertex = NewVertex {
            name: "create_edge_source_vertex".to_string(),
            type_: "create_edge_source_vertex".to_string(),
            created_by: "test".to_string(),
        };

        let target_vertex = NewVertex {
            name: "create_edge_target_vertex".to_string(),
            type_: "create_edge_target_vertex".to_string(),
            created_by: "test".to_string(),
        };

        let mut conn = AsyncPgConnection::establish(&database_url).await.unwrap();
        let source_vertex = crate::api::create_vertex(&mut conn, &source_vertex)
            .await
            .unwrap();

        let target_vertex = crate::api::create_vertex(&mut conn, &target_vertex)
            .await
            .unwrap();

        let new_edge = crate::dto::NewEdge {
            from_vertex_id: source_vertex.id,
            to_vertex_id: target_vertex.id,
            label: "create_edge".to_string(),
            created_by: "test".to_string(),
        };

        let result = crate::api::create_edge(&mut conn, &new_edge).await.unwrap();
        assert!(result.id > 0);
        assert_eq!(result.from_vertex_id, source_vertex.id);
        assert_eq!(result.from_vertex_type, source_vertex.type_);
        assert_eq!(result.to_vertex_id, target_vertex.id);
        assert_eq!(result.to_vertex_type, target_vertex.type_);
        assert_eq!(result.label, "create_edge");
        assert_eq!(result.created_by, "test");
    }

    #[tokio::test]
    async fn test_get_vertex_by_id() {
        dotenvy::from_path(".env").ok();

        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        let new_vertex = NewVertex {
            name: "get_vertex_by_id".to_string(),
            type_: "get_vertex_by_id".to_string(),
            created_by: "test".to_string(),
        };

        let mut conn = AsyncPgConnection::establish(&database_url).await.unwrap();
        let new_vertex = crate::api::create_vertex(&mut conn, &new_vertex)
            .await
            .unwrap();

        let result = crate::api::get_vertex_by_id(&mut conn, new_vertex.id)
            .await
            .unwrap();
        assert_eq!(result.id, new_vertex.id);
        assert_eq!(result.name, "get_vertex_by_id");
        assert_eq!(result.type_, "get_vertex_by_id");
        assert_eq!(result.created_by, "test");
    }

    #[tokio::test]
    async fn test_delete_vertex_by_id_without_relationship() {
        dotenvy::from_path(".env").ok();

        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        let new_vertex = NewVertex {
            name: "delete_vertex_by_id".to_string(),
            type_: "delete_vertex_by_id".to_string(),
            created_by: "test".to_string(),
        };

        let mut conn = AsyncPgConnection::establish(&database_url).await.unwrap();
        let new_vertex = crate::api::create_vertex(&mut conn, &new_vertex)
            .await
            .unwrap();

        let result = crate::api::delete_vertex_by_id(&mut conn, new_vertex.id)
            .await
            .unwrap();
        assert_eq!(result, 1);

        let result = crate::api::get_vertex_by_id(&mut conn, new_vertex.id).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_vertex_by_id_with_relationship() {
        dotenvy::from_path(".env").ok();

        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        let source_vertex = NewVertex {
            name: "delete_vertex_by_id_with_relationship_source_vertex".to_string(),
            type_: "delete_vertex_by_id_with_relationship_source_vertex".to_string(),
            created_by: "test".to_string(),
        };

        let target_vertex = NewVertex {
            name: "delete_vertex_by_id_with_relationship_target_vertex".to_string(),
            type_: "delete_vertex_by_id_with_relationship_target_vertex".to_string(),
            created_by: "test".to_string(),
        };

        let mut conn = AsyncPgConnection::establish(&database_url).await.unwrap();
        let source_vertex = crate::api::create_vertex(&mut conn, &source_vertex)
            .await
            .unwrap();

        let target_vertex = crate::api::create_vertex(&mut conn, &target_vertex)
            .await
            .unwrap();

        let new_edge = crate::dto::NewEdge {
            from_vertex_id: source_vertex.id,
            to_vertex_id: target_vertex.id,
            label: "delete_vertex_by_id_with_relationship".to_string(),
            created_by: "test".to_string(),
        };

        let _ = crate::api::create_edge(&mut conn, &new_edge).await.unwrap();

        let result = crate::api::delete_vertex_by_id(&mut conn, source_vertex.id).await;
        assert!(result.is_ok());

        edge.filter(from_vertex_id.eq(source_vertex.id))
            .select(edge::id)
            .first::<i32>(&mut conn)
            .await
            .expect_err("edge should be deleted");
    }
}
