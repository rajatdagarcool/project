use warp::Filter;
use serde_json::json;

pub fn todo_filter() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone{
//List todo
let todo_base = warp::path("todo");

  let list = todo_base
        .and(warp::get())
        .and(warp::path::end())
        .and_then(todo_list);

    let get = todo_base.and(warp::get()).and(warp::path::param())
        .and_then(todo_get);

    list.or(get)
}
async fn todo_list() -> Result<warp::reply::Json, warp::Rejection>{
    //ToDo- Get from DB

    let todos = json!([
    {"id":  1, "title": "todo 1"},
    {"id": 2, "title": "todo 2"}
    ]);

    let todos = warp::reply::json(&todos);

    Ok(todos)
}

async fn todo_get(id: i64) -> Result<warp::reply::Json, warp::Rejection>{
    //ToDo -  get from DB
    let todo = json!(
    {"id": id, "title": format!("todo {}", id)}
    );

    //serde-json to warp-reply
    let todo = warp::reply::json(&todo);

    Ok(todo)
}

//async fn 
