
use std::future::{ ready, Ready };
use actix_web::{
    HttpRequest,
    FromRequest,
    web,
    dev::{ forward_ready, Service, ServiceRequest, ServiceResponse, Transform, Payload },
    Error as ActixWebError,
};
use futures_util::future::LocalBoxFuture;
use serde::{ Serialize, Deserialize };




#[derive(Serialize, Deserialize, Debug)]
pub struct AuthenticationBody {
    token: String,
}

pub struct Protected;

// Transfom "transforms" a service by wrapping it in another service.
// The Transform implementation's only job is to create new middleware instances that wrap other services.
impl<S, B> Transform<S, ServiceRequest> for Protected
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = ActixWebError>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = ActixWebError;
    type InitError = (); // indicates an error that might occur when creating the middleware instance
    type Transform = ProtectedMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    // new_transform creates a new instance of the middleware Service.
    // The created middleware should wrap the service indicated by the service parameter
    // 
    // returns a Future to allow some asynchronous work to be done while creating the middleware.
    fn new_transform(&self, service: S) -> Self::Future {
	// We only need to create a new object, so we'll use a Ready future to wrap the new middleware inside a future.
	// This is similar to using Javascript's Promise.resolve to place a value inside a Promise
	ready(Ok(ProtectedMiddleware { service }))
    }
}





pub struct ProtectedMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for ProtectedMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = ActixWebError>,
    S::Future: 'static,
    B: 'static, // B type paramter here represents the type of the body returned from the service
{
    type Response = ServiceResponse<B>;
    type Error = ActixWebError;
    // Makes it easier to use an async block without needing to deal with the opaque future
    // types returned by async blocks.
    // LocalBoxFuture is the non-Send version of BoxFuture
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    // Actix calls poll_ready to determine if the service is ready to be invoked.
    // forward_ready! macro delegates this function to the wrapped service
    forward_ready!(service);

    // The call function is where all the "real" functionality goes.
    // You can inspect or mutate the request and response objects as needed,
    // and invoke the wrapped service if appropriate.
    fn call(&self, req: ServiceRequest) -> Self::Future {
	println!("Hi from start. You requested: {}", req.path());

	let fut = self.service.call(req);

	Box::pin(async move {
	    let res = fut.await?;

	    // let body = res.clone().into_body();
	    // let (http_req, payload): (&HttpRequest, &Payload) = res.parts(); 
	    // let auth_body: web::Json<AuthenticationBody> = web::Json::<AuthenticationBody>::from_request(http_req, &mut *payload).await.unwrap();

	    println!("Hi from response");
	    Ok(res)
	})
    }
}
