use axum::body::Body;

#[shuttle_runtime::main]
async fn axum(
    #[shuttle_shared_db::Postgres] postgres: PgPool,
    #[shuttle_secrets::Secrets] secrets: shuttle_secrets::SecretStore,
    #[shuttle_static_folder::StaticFolder] public: PathBuf
) -> shuttle_axum::ShuttleAxum {
    
sqlx::migrate!().run(&postgres).await.expect("Something went wrong while running migrations :(");

    let (stripe_key, mailgun_key, mailgun_url, domain) = grab_secrets(secrets);

    let state = AppState {
        postgres,
        stripe_key,
        mailgun_key,
        mailgun_url,
        domain,
        key: Key::generate(),
    };

        let router = Router::new()
            .nest("/api", api_router)
            .fallback_service(get(|req| async move {
            match ServeDir::new(public).oneshot(req).await {
                Ok(res) => res.map(boxed),
                Err(err) => Response::builder()
                   .status(StatusCode::INTERNAL_SERVER_ERROR)
                 .body(boxed(Body::from(format!("error: {err}"))))
                 .expect("error response"),
         }
     }));


    Ok(router.into())
}

fn grab_secrets(secrets: shuttle_secrets::SecretStore) -> (String, String, String, String) {
    let stripe_key = secrets
        .get("STRIPE_KEY")
        .expect("Couldn't get STRIPE_KEY, did you remember to set it in Secrets.toml?");

    let mailgun_key = secrets
        .get("MAILGUN_KEY")
        .expect("Couldn't get MAILGUN_KEY, did you remember to set it in Secrets.toml?");

    let mailgun_url = secrets
        .get("MAILGUN_URL")
        .expect("Couldn't get MAILGUN_URL, did you remember to set it in Secrets.toml?");

    let domain = secrets
        .get("DOMAIN_URL")
        .expect("Couldn't get DOMAIN_URL, did you remember to set it in Secrets.toml?");

    (stripe_key, mailgun_key, mailgun_url, domain)
}
