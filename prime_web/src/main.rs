use actix_web::{App, HttpResponse, HttpServer, error, get, post, web};

use askama::Template;
use serde::Deserialize;

use crate::repository::RepositoryGenerator;

pub mod repository;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    primes: &'a [i64],
    start: i64,
    end: i64,
}

#[derive(Deserialize)]
struct PrimeForm {
    start: i64,
    end: i64,
}

#[get("/")]
async fn index() -> Result<HttpResponse, actix_web::Error> {
    let primes = [];
    let response_body = IndexTemplate {
        primes: &primes,
        start: 0,
        end: 0,
    };

    Ok(HttpResponse::Ok().body(response_body.render().unwrap()))
}

#[post("/generate")]
async fn generate(form: web::Form<PrimeForm>) -> Result<HttpResponse, actix_web::Error> {
    let start = form.start.max(1);
    let end = form.end.max(1);

    let prime_genarator = repository::Primes::new("primes.db");

    let primes = match prime_genarator.extract_prime(start, end)
    {
        Ok(x) => x,
        _ => return Err(error::ErrorInternalServerError("something went wrong")),
    };

    let response_body = IndexTemplate {
        primes: &primes,
        start: start,
        end: end,
    };

    Ok(HttpResponse::Ok().body(response_body.render().unwrap()))
}

#[actix_web::main]
async fn main() -> Result<(), actix_web::Error> {
    HttpServer::new(move || App::new().service(index).service(generate))
        .bind("0.0.0.0:8080")?
        .run()
        .await?;

    Ok(())
}
