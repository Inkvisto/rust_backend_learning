#[derive(Deserialize)]
pub struct RegisterDetails {
name: String,
email: String,
password: String,
}

#[derive(Deserialize)]
pub struct LoginDetails {
email: String,
password: String,
}


pub async fn register(
    State(state): State<AppState>,
    Json(newuser): Json<RegisterDetails>,
) -> impl IntoResponse {
// attempt to hash the password from request body - this is required as 
// otherwise plaintext passwords in the database is unsafe
let hashed_password = bcrypt::hash(newuser.password, 10).unwrap();

// set up query
let query = sqlx::query("INSERT INTO users (name, email, password) values ($1, $2, $3)")
.bind(newuser.name)
.bind(newuser.email)
.bind(hashed_password)
.execute(&state.postgres);

// if the query is OK, return the Created status code along with a response to confirm it
// if not, return a Bad Request status code along with the error code
match query.await {
Ok(_) => (StatusCode::CREATED, "Account created!".to_string()).into_response(),
Err(e) => (
    StatusCode::BAD_REQUEST,
    format!("Something went wrong: {e}"),
).into_response(),
}
}

pub async fn login(
    State(state): State<AppState>,
    jar: PrivateCookieJar,
    Json(login): Json<LoginDetails>,
) -> Result<(PrivateCookieJar, StatusCode), StatusCode> {
// attempt to find a user based on what the request body email is
let query = sqlx::query("SELECT * FROM users WHERE email = $1")
.bind(&login.email)
.fetch_one(&state.postgres);

// if the query is OK, attempt to verify bcrypt hash
match query.await {
Ok(res) => {
if bcrypt::verify(login.password, res.get("password")).is_err() {
    return Err(StatusCode::BAD_REQUEST);
}

// if the hash matches, create a session ID and attempt to write a session to the database
let session_id = rand::random::<u64>().to_string();

// create the session entry in our database table
sqlx::query("INSERT INTO sessions (session_id, user_id) VALUES ($1, $2) ON CONFLICT (user_id)
 DO UPDATE SET session_id = EXCLUDED.session_id")
.bind(&session_id)
.bind(res.get::<i32, _>("id"))
.execute(&state.postgres)
.await
.expect("Couldn't insert session :(");

// build a cookie and add it to the cookiejar as a response, which sends a cookie to the user
// we will be using this later on to validate a user session
let cookie = Cookie::build("foo", session_id)
.secure(true)
.same_site(SameSite::Strict)
.http_only(true)
.path("/")
.max_age(Duration::WEEK)
.finish();

// return cookie and OK status
Ok((jar.add(cookie), StatusCode::OK))
}

// return only Bad Request - this is somewhat vague, but helps deter would-be
// hackers and is good for security as we know what the control flow is
    Err(_) => Err(StatusCode::BAD_REQUEST),
    }
}

pub async fn validate_session<B>(
    jar: PrivateCookieJar,
    State(state): State<AppState>,
    request: Request<B>,
    next: Next<B>,
) -> (PrivateCookieJar, Response) {
// grab token value by mapping the token and getting the value 
let Some(cookie) = jar.get("foo").map(|cookie| cookie.value().to_owned()) else {
// if the cookie doesn't exist or has no value, print a line and return Forbidden
println!("Couldn't find a cookie in the jar");
return (jar,(StatusCode::FORBIDDEN, "Forbidden!".to_string()).into_response())
};

// set up the query to match session against what our cookie session ID value is
let find_session = sqlx::query("SELECT * FROM sessions WHERE session_id = $1")
.bind(cookie)
.execute(&state.postgres)
.await;

// if it matches, return the jar and run the request the user wants to make
// if not, return 403 Forbidden
match find_session {
Ok(_) => (jar, next.run(request).await),
Err(_) => (
    jar,
    (StatusCode::FORBIDDEN, "Forbidden!".to_string()).into_response(),
    ),
    }
}

pub async fn logout(
    State(state): State<AppState>,
    jar: PrivateCookieJar,
) -> Result<PrivateCookieJar, StatusCode> {
let Some(cookie) = jar.get("sessionid").map(|cookie| cookie.value().to_owned()) else {
     return Ok(jar)
};

let query = sqlx::query("DELETE FROM sessions WHERE session_id = $1")
.bind(cookie)
.execute(&state.postgres);

match query.await {
Ok(_) => Ok(jar.remove(Cookie::named("foo"))),
Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
}
}

