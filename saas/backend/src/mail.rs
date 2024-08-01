// define a struct that takes an email
#[derive(Deserialize, Serialize)]
pub struct EmailRequest {
    email: String,
}

pub async fn subscribe(
    State(state): State<AppState>,
    Json(req): Json<EmailRequest>,
) -> Result<StatusCode, StatusCode> {
// initialise a reqwest non-blocking client
let ctx = reqwest::Client::new();

// create a string for the correct API endpoint we'll be posting to
let api_endpoint = format!("https://api.mailgun.net/v3/lists/mail@{}/members", &state.mailgun_url);

let params = sub_params(req.email);
let post = ctx
.post(api_endpoint)
.basic_auth("api", Some(&state.mailgun_key))
.form(&params);

match post.send().await {
    Ok(_) => Ok(StatusCode::OK),
    Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
  }
}

// create a hashmap of headers that we'll be using in our POST request
fn sub_params(recipient: String) -> HashMap<&'static str, String> {
let mut params = HashMap::new();

params.insert("address", recipient);
params.insert("subscribed", "True".to_string());

params
}
