pub fn create_api_router(state: AppState) -> Router {

let cors = CorsLayer::new()
.allow_credentials(true)
.allow_methods(vec![Method::GET, Method::POST, Method::PUT, Method::DELETE])
.allow_headers(vec![ORIGIN, AUTHORIZATION, ACCEPT])
.allow_origin(&state.domain_url.parse().unwrap());

let payments_router = Router::new().route("/pay", post(create_checkout));

let customers_router = Router::new()
.route("/", post(get_all_customers))
.route(
    "/:id",
    post(get_one_customer)
    .put(edit_customer)
    .delete(destroy_customer),
)
        .route("/create", post(create_customer));

let deals_router = Router::new()
.route("/", post(get_all_deals))
.route(
      "/:id",
      post(get_one_deal)
     .put(edit_deal)
     .delete(destroy_deal),
)
.route("/create", post(create_deal));

let auth_router = Router::new()
.route("/register", post(register))
.route("/login", post(login))
.route("/logout", get(logout));

Router::new()
.nest("/customers", customers_router)
.nest("/deals", deals_router)
.nest("/payments", payments_router)
.nest("/auth", auth_router)
.route("/subscribe", post(subscribe))
.with_state(state)
.layer(cors)
}
