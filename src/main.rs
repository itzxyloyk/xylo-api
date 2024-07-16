use warp::Filter;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Serialize, Deserialize, Clone)]
struct User {
    id: u32,
    name: String,
    email: String,
}

type UsersDb = Arc<Mutex<Vec<User>>>;

#[tokio::main]

async fn main() {
    let users = UsersDb::new(Mutex::new(vec![
        User { id: 1, name: "John Doe".to_string(), email: "john@example.com".to_string() },
        User { id: 2, name: "Jane Smith".to_string(), email: "jane@example.com".to_string() },
    ]));
    
    let get_users = warp::path("users")
        .and(warp::get())
        .and(with_users(users.clone()))
        .and_then(get_users_handler);
        
    let post_user = warp::path("users")
        .and(warp::post())
        .and(json_body())
        .and(with_users(users.clone()))
        .and_then(post_user_handler);

    let routes = get_users.or(post_user);
    
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

async fn get_users_handler(users_db: UsersDb) -> Result<impl warp::Reply, warp::Rejection> {
    let users = users_db.lock().unwrap();
    Ok(warp::reply::json(&*users))
}

async fn post_user_handler(new_user: User, users_db: UsersDb) -> Result<impl warp::Reply, warp::Rejection> {
    let mut users = users_db.lock().unwrap();
    users.push(new_user);
    Ok(warp::reply::json(&*users))
}

fn with_users(users_db: UsersDb) -> impl Filter<Extract = (UsersDb,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || users_db.clone())
}

fn json_body() -> impl Filter<Extract = (User,), Error = warp::Rejection> + Clone {
    warp::body::json()
}
