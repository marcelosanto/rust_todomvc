use crate::{
    model::{
        db::init_db,
        todo::{TodoPatch, TodoStatus},
    },
    security::utx_from_token,
};

use super::TodoMac;

#[tokio::test]
async fn model_todo_create() -> Result<(), Box<dyn std::error::Error>> {
    // -- FIXTURE
    let db = init_db().await?;

    let utx = utx_from_token("123").await?;

    let data_fx = TodoPatch {
        title: Some("text - model_todo_create 1".to_string()),
        ..Default::default()
    };

    // -- ACTION
    let todo_created = TodoMac::create(&db, &utx, data_fx.clone()).await?;

    // -- CHECK
    assert!(todo_created.id >= 100, "Id should be >= 100");
    assert_eq!(data_fx.title.unwrap(), todo_created.title);
    assert_eq!(TodoStatus::Open, todo_created.status);

    Ok(())
}

#[tokio::test]
async fn model_todo_get() -> Result<(), Box<dyn std::error::Error>> {
    let db = init_db().await?;

    let utx = utx_from_token("123").await?;

    let todo = TodoMac::get(&db, &utx, 100).await?;

    assert_eq!(100, todo.id);
    assert_eq!("todo 100", todo.title);
    assert_eq!(TodoStatus::Close, todo.status);

    Ok(())
}

#[tokio::test]
async fn model_todo_update() -> Result<(), Box<dyn std::error::Error>> {
    let db = init_db().await?;

    let utx = utx_from_token("123").await?;

    let data_fx = TodoPatch {
        title: Some("test - model_todo_update_ok 1".to_string()),
        ..Default::default()
    };

    let todo_fx = TodoMac::create(&db, &utx, data_fx.clone()).await?;

    let update_data_fx = TodoPatch {
        title: Some("test - model_todo_update_ok 2".to_string()),
        ..Default::default()
    };

    let todo = TodoMac::update(&db, &utx, todo_fx.id, update_data_fx.clone()).await?;

    let todos = TodoMac::list(&db, &utx).await?;

    assert_eq!(3, todos.len());
    assert_eq!(todo_fx.id, todo.id);
    assert_eq!(update_data_fx.title.unwrap(), todo.title);

    Ok(())
}

#[tokio::test]
async fn model_todo_list() -> Result<(), Box<dyn std::error::Error>> {
    // -- FIXTURE
    let db = init_db().await?;

    let utx = utx_from_token("123").await?;

    // -- Action
    let todos = TodoMac::list(&db, &utx).await?;

    // -- CHECK
    assert_eq!(2, todos.len());

    //todo 101
    assert_eq!(101, todos[0].id);
    assert_eq!(123, todos[0].cid);
    assert_eq!("todo 101", todos[0].title);

    //todo 100
    assert_eq!(100, todos[1].id);
    assert_eq!(123, todos[1].cid);
    assert_eq!("todo 100", todos[1].title);

    Ok(())
}
