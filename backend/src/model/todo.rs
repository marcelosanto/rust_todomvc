use warp::filters::query;

use super::db::Db;
use crate::model;

// region: Todo Types
#[derive(sqlx::FromRow, Debug, Clone)]
pub struct Todo {
    pub id: i64,
    pub cid: i64,
    pub title: String,
}

pub struct TodoPatch {
    pub cid: Option<i64>,
    pub title: Option<String>,
}

// endregion: Todo Types

// region: TodoMac
#[derive(Debug)]
pub struct TodoMac;

impl TodoMac {
    pub async fn create(db: &Db, data: TodoPatch) -> Result<Todo, model::Error> {
        let sql = "INSERTO INTO todo (cid, title) VALUES ($1, $2) returning id, cid, title";

        let query = sqlx::query_as::<_, Todo>(&sql)
            .bind(123_i64)
            .bind(data.title.unwrap_or_else(|| "untitled".to_string()));

        let todo = query.fetch_one(db).await?;

        Ok(todo)
    }

    pub async fn list(db: &Db) -> Result<Vec<Todo>, model::Error> {
        let sql = "SELECT id, cid, title FROM todo ORDER BY id DESC";

        // build the sqlx-query
        let query = sqlx::query_as(&sql);

        // execute the query
        let todos = query.fetch_all(db).await?;

        Ok(todos)
    }
}

// endregion: TodoMac

#[cfg(test)]
#[path = "../_tests/model_todo.rs"]
mod tests;