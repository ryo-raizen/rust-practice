use actix_files as fs;
use actix_web::http::header;
use actix_web::{
    web, App,
    HttpRequest, HttpResponse, HttpServer,
};
use std::{env, io};
use webbrowser;

#[actix_web::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    let server = HttpServer::new(|| {
        App::new()
            .service(fs::Files::new("/static", "static").show_files_listing())
            // redirect
            .service(web::resource("/").route(web::get().to(|req: HttpRequest| {
                println!("{:?}", req);
                HttpResponse::Found()
                    .header(header::LOCATION, "static/index.html")
                    .finish()
            })))
            // default
            .default_service(web::resource("").route(web::get().to(|req: HttpRequest| {
                println!("{:?}", req);
                HttpResponse::Found()
                    .header(header::LOCATION, "static/index.html")
                    .finish()
            })))
    })
    .bind("127.0.0.1:8080")?
    .run();
    if webbrowser::open("http://127.0.0.1:8080/").is_ok() {
        println!("local server open!");
    }
    server.await
}


