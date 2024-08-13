use crate::{
    dto::{NewEdge, NewVertex},
    error::Error,
    model::{Edge, Vertex},
};
use diesel::{ExpressionMethods, QueryDsl, SelectableHelper};
use diesel_async::{AsyncPgConnection, RunQueryDsl};

async fn create_vertex(
    conn: &mut AsyncPgConnection,
    new_vertex: NewVertex,
) -> Result<Vertex, Error> {
    use crate::schema::vertex::dsl::*;

    let result = diesel::insert_into(vertex)
        .values(&new_vertex)
        .returning(Vertex::as_returning())
        .get_result(conn)
        .await?;

    Ok(result)
}

async fn create_edge(conn: &mut AsyncPgConnection, new_edge: NewEdge) -> Result<Edge, Error> {
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

    let new_edge = NewEdge {
        from_vertex_type: source_vertex_type,
        to_vertex_type: target_vertex_type,
        ..new_edge
    };

    let result = diesel::insert_into(edge)
        .values(&new_edge)
        .returning(Edge::as_returning())
        .get_result(conn)
        .await?;

    Ok(result)
}
