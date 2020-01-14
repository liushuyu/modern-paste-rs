use actix_files;
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
    let s = tmpl.render("paste/post.html", &ctx).map_err(|_| {
        HttpResponse::InternalServerError()
            .content_type("text/html")
            .body(error(tmpl, &config))
    })?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

async fn archive(
    tmpl: web::Data<tera::Tera>,
    config: web::Data<config::Config>,
) -> Result<HttpResponse, Error> {
    let mut ctx = tera::Context::new();
    let config_raw: &config::Config = &*config;
    ctx.insert("config", config_raw);
    let s = tmpl
        .render("paste/archive.html", &ctx)
        .map_err(|_| error::ErrorInternalServerError(error(tmpl, &config)))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

fn error(tmpl: web::Data<tera::Tera>, config: &config::Config) -> String {
    let mut ctx = tera::Context::new();
    ctx.insert("config", config);
    let s = tmpl
        .render("error/internal_server_error.html", &ctx)
        .unwrap_or("Template rendering problem".to_string());
    s
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap();
        let config = config::Config::new();

        App::new()
            .data(tera)
            .data(config)
            .wrap(middleware::Logger::default()) // enable logger
            .service(web::resource("/").route(web::get().to(index)))
            .service(web::resource("/paste/new").route(web::get().to(index)))
            .service(web::resource("/archive").route(web::get().to(archive)))
            .service(actix_files::Files::new("/static", "static"))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
