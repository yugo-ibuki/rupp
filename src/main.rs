use serde::{Deserialize, Serialize};
use std::sync::Arc;
use warp::Filter;
use tokio::sync::RwLock;

// Todo構造体を定義します
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Todo {
    id: u64,
    title: String,
    completed: bool,
}

#[tokio::main]
async fn main() {
    let todos = Arc::new(RwLock::new(Vec::<Todo>::new()));
    let todos_filter = warp::any().map(move || Arc::clone(&todos));

    let get_todos = warp::get()
        .and(warp::path("todos"))
        .and(todos_filter.clone())
        .and_then(get_todos_handler);

    let create_todo = warp::post()
        .and(warp::path("todos"))
        .and(warp::body::json())
        .and(todos_filter.clone())
        .and_then(create_todo_handler);

    let static_files = warp::path("static")
        .and(warp::fs::dir("./static"))
        .with(warp::log("static_files"));

    let routes = get_todos.or(create_todo).or(static_files);

    println!("サーバーが起動しました。http://localhost:3030 にアクセスしてください。");
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

async fn get_todos_handler(todos: Arc<RwLock<Vec<Todo>>>) -> Result<impl warp::Reply, warp::Rejection> {
    let todos = todos.read().await;
    Ok(warp::reply::json(&*todos))
}

async fn create_todo_handler(new_todo: Todo, todos: Arc<RwLock<Vec<Todo>>>) -> Result<impl warp::Reply, warp::Rejection> {
    let mut todos = todos.write().await;
    todos.push(new_todo);
    Ok(warp::reply::with_status("Created", warp::http::StatusCode::CREATED))
}
