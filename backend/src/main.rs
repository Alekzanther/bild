extern crate actix_files;
extern crate actix_rt;
extern crate actix_utils;
extern crate actix_web;
extern crate blake2;
extern crate bytes;
extern crate diesel;
extern crate dotenv;
extern crate env_logger;
extern crate exif;
extern crate glob;
extern crate image;
extern crate juniper;
extern crate log;
extern crate motiv;
extern crate r2d2;
extern crate serde;
extern crate toml;

use std::{env, io, thread};

use actix_web::{middleware, App, HttpServer};
use log::warn;
use motiv::config;
use motiv::data::db::get_pool;
use motiv::endpoints::web_endpoints;
use motiv::services::indexer;
use motiv::services::worker;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    logging_setup();
    let cfg_path = "./motiv.toml";
    let cfg = config::read_config(cfg_path.to_string());
    let cfg = match cfg {
        Ok(result) => result,
        Err(e) => return Err(e),
    };

    // Instantiate a new connection pool
    let pool = get_pool();

    // Start indexing
    if let Some(media_paths) = cfg.media {
        let indexer_pool = pool.clone();
        thread::spawn(move || {
            indexer::index_media(&indexer_pool.get().unwrap(), media_paths);
        });
    } else {
        warn!("No media paths configured in {}, not indexing", cfg_path);
    }

    let worker_db_pool = pool.clone();
    thread::spawn(move || loop {
        worker::process_unprocessed(&worker_db_pool.get().unwrap());
        thread::sleep(std::time::Duration::from_secs(5));
    });

    //set bindstr from cfg (fallback 5000)
    let bindstr = "0.0.0.0:".to_string() + &(cfg.port.unwrap_or(5000)).to_string();
    println!("Starting up on {}", bindstr);

    // Start up the server, passing in (a) the connection pool
    // to make it available to all endpoints and (b) the configuration
    // function that adds the /graphql logic.
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .configure(web_endpoints)
            .default_service(actix_files::Files::new("/", "./frontend/build"))
    })
    .bind(bindstr)?
    .run()
    .await
}

// TODO: more fine-grained logging setup
fn logging_setup() {
    env::set_var("RUST_LOG", "motiv=debug,actix_web=info");
    env_logger::init();
}