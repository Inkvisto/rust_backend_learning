use axum::{
    async_trait, body::Body, extract::{rejection::JsonRejection, FromRequest, Path, Query, Request}, http::{HeaderMap, HeaderValue, Method, StatusCode}, middleware::Next, response::Response, routing::{get, post}, Extension, Json, Router
};
use validator::{Validate, ValidationErrors};
use serde::{Deserialize, Serialize};
use tower_http::cors::{Any, CorsLayer};
pub fn create_router() -> Router<()> {
    let cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET, Method::POST])
        // allow requests from any origin
        .allow_origin("http://example.com".parse::<HeaderValue>().unwrap())
        .allow_headers(Any);

    let shared_data = SharedData {
        message: "Some shared data".to_owned(),
    };

    Router::new()

        .route("/", get(hello))
        .route("/:id", get(path_vars))
        .route("/user", get(path_query))
        .route("/always_errors", get(always_errors))
        .route("/", post(str_token))
        .route("/user_valid", post(custom_json_extractor))
        .route("/mw_message", get(mw_message))
        .layer(cors)
        .layer(Extension(shared_data))
}

async fn hello() -> String {
    String::from("Hello world!")
}

async fn path_vars(Path(user_id): Path<String>) -> String {
    user_id
}

#[derive(Serialize, Deserialize)]
struct QueryParams {
    name: String,
}

async fn path_query(Query(user): Query<QueryParams>) -> String {
    user.name
}

async fn always_errors() -> Result<(), StatusCode> {
    Err(StatusCode::IM_A_TEAPOT)
}

#[derive(Serialize, Deserialize)]
struct User {
    email: String,
    name: String,
}

async fn str_token(header_map: HeaderMap, Json(body): Json<User>) -> Json<User> {
    dbg!(header_map);
    Json(body)
}

#[derive(Clone)]
pub struct SharedData {
    pub message: String,
}

async fn mw_message(Extension(shared_data): Extension<SharedData>) -> String {
    shared_data.message
}

async fn mw_custom(mut req: Request, next: Next) -> Result<Response, StatusCode> {
    let headers = req.headers();
    let message = headers
        .get("message")
        .ok_or_else(|| StatusCode::BAD_REQUEST)?;
    let ext = req.extensions_mut();
        ext.insert(SharedData{ message: "some".to_owned()});
    
    Ok(next.run(req).await)
}



#[derive(Deserialize, Debug, Validate)]
pub struct RequestUser {
    #[validate(email(message = "must be a valid email"))]
    pub username: String,
    #[validate(length(min = 8, message = "must have at least 8 characters"))]
    pub password: String,
}

#[async_trait]
impl<B> FromRequest<B> for RequestUser
where
    B: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request(req: axum::http::Request<Body>, _state: &B) -> Result<Self, Self::Rejection> {
        let Json(user): Json<RequestUser> = Json::<RequestUser>::from_request(req, _state)
            .await
            .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid JSON: {}", e)))?;

        user.validate()
            .map_err(|e: ValidationErrors| (StatusCode::BAD_REQUEST, format!("{}", e)))?;

        Ok(user)
    }
}

pub async fn custom_json_extractor(user: RequestUser) {
    dbg!(user);
}
