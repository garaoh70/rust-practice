use actix_web::{App, HttpResponse, HttpServer, error, get, post, web};
use askama::Template;
use clap::Parser;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use serde::Deserialize;

type SqlitePool = Pool<SqliteConnectionManager>;

use crate::repository::RepositoryGenerator;

pub mod cli;
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
async fn generate(
    pool: web::Data<SqlitePool>,
    form: web::Form<PrimeForm>,
) -> Result<HttpResponse, actix_web::Error> {
    let start = form.start.max(1);
    let end = form.end.max(1);

    if start > end || end - start > 10000000 {
        return Err(error::ErrorBadRequest("invalid range"));
    }

    let prime_genarator = repository::Primes;
    let pool = pool.clone();

    let primes = web::block(move || {
        let connection = pool
            .get()
            .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
        prime_genarator.extract_prime(&connection, start, end)
    })
    .await
    .map_err(|_| error::ErrorInternalServerError("blocking error"))?
    .map_err(|_| error::ErrorInternalServerError("db error"))?;

    let response_body = IndexTemplate {
        primes: &primes,
        start: start,
        end: end,
    };

    let body = response_body
        .render()
        .map_err(|_| error::ErrorInternalServerError("template error"))?;

    Ok(HttpResponse::Ok().body(body))
}

#[actix_web::main]
async fn main() -> Result<(), actix_web::Error> {
    let args = cli::Arguments::parse();

    let manager = SqliteConnectionManager::file(args.database);
    let pool = Pool::new(manager)
        .map_err(|_| error::ErrorInternalServerError("Database Connection Error"))?;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(index)
            .service(generate)
    })
    .bind(args.bind)?
    .run()
    .await?;

    Ok(())
}
