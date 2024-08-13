use crate::{
    dto::{InsertableNewEdge, InsertableNewVertex, NewEdge, NewVertex},
    error::Error,
    model::{Edge, Vertex},
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

#[cfg(test)]
mod tests {

    use diesel_async::{AsyncConnection, AsyncPgConnection};

    use crate::dto::NewVertex;

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
}
