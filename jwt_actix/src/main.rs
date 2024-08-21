use std::io::Result;
use actix_web::{ web, App, HttpServer };

mod scopes;
mod extractors;
use scopes::user::user_scope;

#[actix_web::main]
async fn main() -> Result<()> {
    HttpServer::new(|| {
	App::new()
	    .app_data(web::Data::<String>::new("secret".to_owned()))
	    .service(user_scope())
    }).bind(("127.0.0.1", 8080))?
	.run()
	.await
}
