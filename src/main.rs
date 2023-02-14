use axum::{
    routing::{get, post},
    http::StatusCode,
    response::IntoResponse,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // 启动日志记录
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/",get(root))
        .route("/create_user", post(create_user));
    //服务端口
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    //启动服务
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

//handler函数
async fn root() -> &'static str {
    "hello,world"
} 
//handler函数
async fn create_user(
    Json(payload): Json<CreateUser>
) ->  impl IntoResponse {
    let user  = User {
        id: 1,
        username: payload.username,
    };
    (StatusCode::CREATED, Json(user))

}


#[derive(Deserialize)]
struct CreateUser {
    username: String
}


#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}