use sqlb::HasFields;
use warp::filters::query;

use super::db::Db;
use crate::model;

// region: Todo Types
#[derive(sqlx::FromRow, Debug, Clone)]
pub struct Todo {
    pub id: i64,
    pub cid: i64,
    pub title: String,
    pub status: TodoStatus
}

#[derive(Default, Debug,Clone, sqlb::Fields)]
pub struct TodoPatch {
    pub title: Option<String>,
    pub status: Option<TodoStatus>
}

#[derive(sqlx::Type,Debug, Clone, PartialEq, Eq)]
#[sqlx(type_name = "todo_status_enum")]
#[sqlx(rename_all = "lowercase")]
pub enum TodoStatus {
    Open,
    Close
}

sqlb::bindable!(TodoStatus);
// endregion: Todo Types

// region: TodoMac
#[derive(Debug)]
pub struct TodoMac;

impl TodoMac {
    pub async fn create(db: &Db, data: TodoPatch) -> Result<Todo, model::Error> {
        // let sql = "INSERT INTO todo (cid, title) VALUES ($1, $2) returning id, cid, title, status";

        // let query = sqlx::query_as::<_, Todo>(&sql)
        //     .bind(123_i64)
        //     .bind(data.title.unwrap_or_else(|| "untitled".to_string()));

        let sb = sqlb::insert()
        .table("todo")
        .data(data.all_fields())
        .returning(&["id", "cid", "title", "status"]);
        
        let todo = sb.fetch_one(db).await?;

        Ok(todo)
    }

    pub async fn list(db: &Db) -> Result<Vec<Todo>, model::Error> {
        let sql = "SELECT id, cid, title, status FROM todo ORDER BY id DESC";

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
