#![allow(unused)]

mod reqwest_wrapper;


use anyhow::Result;
use axum::{Json,http::HeaderMap};
use reqwest::Response;
use reqwest_wrapper::HttpClient;
use serde::Deserialize;
use serde_json::json;

#[derive(Debug, Deserialize)]
struct LoginResponse{
    username: String,
    pwd: String,
}


#[tokio::test]
async fn quick_dev() -> Result<()> {

    let client = HttpClient::new("http://localhost:8080", HeaderMap::new())?;
    let res  = HttpClient::get(&client, "/hello?name=Egor").await?;
    let res2  = HttpClient::get(&client, "/hello2/Lucy").await?;
    let res_dirs  = HttpClient::get(
        &client, 
        "/src/main.rs"
    ).await?;

    let req_login = HttpClient::post(
        &client,
        "/api/login", 
        &json!({
            "username": "demo1",
            "pwd": "welcome"})
    ).await?;


    let req_create_ticket = HttpClient::post(
        &client,
        "/api/tickets",
        &json!({
            "title": "Ticket AAA"
        })    
    ).await?;

    let tickets  = HttpClient::get(&client, "/api/tickets").await?;
    
    
    
    dbg!(res);
    dbg!(res2);
    // dbg!(res_dirs);
    dbg!(req_login.text().await?);
    dbg!(req_create_ticket.text().await?);
    dbg!(tickets);
    Ok(())
}
