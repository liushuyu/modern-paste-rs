use std::collections::HashMap;

use actix_web::{error, middleware, web, App, Error, HttpResponse, HttpServer};
use tera::Tera;

mod config;

async fn index(
    tmpl: web::Data<tera::Tera>,
    config: web::Data<config::Config>,
) -> Result<HttpResponse, Error> {
    let mut ctx = tera::Context::new();
    let config_raw: &config::Config = &*config;
    ctx.insert("config", config_raw);
    let s = tmpl.render("paste/post.html", &ctx)
            .map_err(|_| error::ErrorInternalServerError("Template error"))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

async fn error(
    tmpl: web::Data<tera::Tera>,
    config: web::Data<config::Config>,
) -> Result<HttpResponse, Error> {
    let mut ctx = tera::Context::new();
    let config_raw: &config::Config = &*config;
    ctx.insert("config", config_raw);
    let s = tmpl.render("error/internal_server_error.html", &ctx)
    .map_err(|_| error::ErrorInternalServerError("Template error"))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        let tera =
            Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap();
        let config = config::Config::new();

        App::new()
            .data(tera)
            .data(config)
            .wrap(middleware::Logger::default()) // enable logger
            .service(web::resource("/").route(web::get().to(index)))
            .service(web::resource("/error").route(web::get().to(error)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
