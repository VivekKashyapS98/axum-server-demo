use axum::{extract::Query, response::Html, routing::get, Router};
use std::{collections::HashMap, net::SocketAddr};

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new()
        .route("/", get(handler))
        .route("/add", get(add))
        .route("/multiply", get(multiply))
        .route("/divide", get(divide))
        .route("/modulus", get(modulus));

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, Rusticians!</h1>")
}

async fn add(Query(params): Query<HashMap<String, String>>) -> Html<String> {
    let sum = &params["x"].parse::<i32>().unwrap() + &params["y"].parse::<i32>().unwrap();
    Html(format!("<h1>{}</h1>", sum))
}

async fn multiply(Query(params): Query<HashMap<String, String>>) -> Html<String> {
    let product = &params["x"].parse::<i32>().unwrap() * &params["y"].parse::<i32>().unwrap();
    Html(format!("<h1>{}</h1>", product))
}

async fn divide(Query(params): Query<HashMap<String, String>>) -> Html<String> {
    let quot = &params["x"].parse::<f32>().unwrap() / &params["y"].parse::<f32>().unwrap();
    Html(format!("<h1>{}</h1>", quot))
}

async fn modulus(Query(params): Query<HashMap<String, String>>) -> Html<String> {
    let ans = &params["x"].parse::<f32>().unwrap() % &params["y"].parse::<f32>().unwrap();
    Html(format!("<h1>{}</h1>", ans))
}
