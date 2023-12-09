pub mod models;
use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use rbatis::RBatis;
use std::net::SocketAddr;

use crate::models::company::Company;
#[derive(Clone)]
struct AppState {
    rb: RBatis,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    //  fast_log::init(fast_log::Config::new().console()).expect("rbatis init fail");
    let rb = RBatis::new();

    let db_url: String = std::env::var("DATABASE_URL")
        .unwrap()
        .parse()
        .expect("DATABASE_URL must set");

    rb.link(rbdc_pg::PgDriver {}, db_url.as_str())
        .await
        .unwrap();

    let state = AppState { rb };

    tracing::info!("--> printing all companies");

    let app = Router::new()
        .route("/", get(get_all_companies))
        .route("/create", post(insert_random_company))
        .with_state(state);

    let port: u16 = std::env::var("PORT")
        .unwrap_or("3000".into())
        .parse()
        .expect("failed to convert to number");

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn insert_random_company(State(state): State<AppState>) -> impl IntoResponse {
    use fake::faker::company::raw::*;
    use fake::locales::*;
    use fake::Fake;

    let fake_inserted_company = Company::insert(
        &state.rb,
        &Company {
            id: Some(cuid::cuid2()),
            name: Some(CompanyName(EN).fake()),
        },
    )
    .await;

    if fake_inserted_company.is_ok() {
        return (
            StatusCode::OK,
            Json(serde_json::json!({
                "message": "Company inserted succesfully"
            })),
        );
    };

    (
        StatusCode::BAD_REQUEST,
        Json(serde_json::json!({
            "error": "Unable to create company"
        })),
    )
}

async fn get_all_companies(State(state): State<AppState>) -> impl IntoResponse {
    let companies = Company::select_all(&state.rb).await;

    (
        StatusCode::OK,
        (serde_json::to_string(&companies.unwrap()).unwrap()),
    )
}
