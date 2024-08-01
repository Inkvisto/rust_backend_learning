
#[derive(Deserialize, Serialize)]
pub struct PaymentInfo {
name: String,
email: String,
card: String,
expyear: i32,
expmonth: i32,
cvc: String,
}

fn create_checkout_params(customer_id: CustomerId) -> CreateSubscription<'static> {
// create a new subscription object with the customer ID (passed in from endpoint function)
    let mut params = CreateSubscription::new(customer_id);
    params.items = Some(vec![CreateSubscriptionItems {
// price ID goes below
        price: Some(<PRICE_ID_GOES_HERE>.to_string()),
        ..Default::default()
    }]);
    params.expand = &["items", "items.data.price.product", "schedule"];

    params
}

pub async fn create_customer(
    ctx: Client,
    name: String, 
    email: String) -> Customer {

Customer::create(
&ctx,
CreateCustomer {
    name: Some(name),
    email: Some(email),
    ..Default::default()
        },
    )
.await
.unwrap()
}

pub async fn create_payment_method(
    ctx: Client, 
    card: String, 
    expyear: i32, 
    expmonth: i32, 
    cvc: String) -> PaymentMethod {
PaymentMethod::create(
&ctx,
CreatePaymentMethod {
    type_: Some(PaymentMethodTypeFilter::Card),
    card: Some(CreatePaymentMethodCardUnion::CardDetailsParams(
        CardDetailsParams {
        number: card,
        exp_year: expyear,
        exp_month: expmonth,
        cvc: Some(cvc),
        },
     )),
     ..Default::default()
     },
)
.await
.unwrap()
}


pub async fn create_checkout(State(state): State<AppState>, Json(req): Json<PaymentInfo>)
 -> Result<StatusCode, StatusCode> {
let ctx = stripe::Client::new(&state.stripe_key);

// Create a new customer
let customer = create_customer(ctx, req.name, req.email).await;

let payment_method = {
// create payment method
let pm = create_payment_method(ctx, req.card, req.expyear, req.expmonth, req.cvc).await;

// attach the payment method to our customer
PaymentMethod::attach(
    &ctx,
    &pm.id,
    AttachPaymentMethod {
        customer: customer.id.clone(),
        },
    )
    .await
    .unwrap();

pm
};

// initialise checkout parameters using the id of the customer we created
let mut params = create_checkout_params(customer.id);

// make the default payment method for the parameters the payment method we created earlier
params.default_payment_method = Some(&payment_method.id);

// attempt to connect to Stripe and actually process the subscription creation
// if it fails, return internal server error
let Ok(_) = Subscription::create(&ctx, params).await else {
   return Err(StatusCode::INTERNAL_SERVER_ERROR)
};

Ok(StatusCode::OK)
}
