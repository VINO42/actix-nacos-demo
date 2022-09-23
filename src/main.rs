use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use log::info;
use log4rs;
use nacos_rust_client::client::naming_client::NamingClient;
use nacos_rust_client::client::naming_client::Instance;



#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    log4rs::init_file("log.yml", Default::default()).unwrap();
    let addr="127.0.0.1:9999";
    info!("Start Server Address : {}", addr);
    info!("Server is starting .... ");
    let naming_client = NamingClient::new_with_addrs("120.27.9.210:30008","dev".to_string(),None);
 
    let instance = Instance::new_simple("127.0.0.1",9999,"rust-nacos-demo","DEFAULT_GROUP");
    naming_client.register(instance);

     HttpServer::new(|| {
        App::new()
            .service(hello)

    })
    .workers(8)
    .shutdown_timeout(30)
    .bind(addr.to_string())?
    .run()
    .await
}
