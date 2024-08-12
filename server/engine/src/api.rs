use crate::{dto::NewVertex, error::Error, model::Vertex};
use diesel::SelectableHelper;
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
