use actix_web::{
    self, middleware, web, App, Error, HttpResponse, HttpServer,
};
use actix_http::{
    Response
};
use futures::Future;
use bytes;

fn proxy_service() -> impl Future<Item=bytes::Bytes,Error=Error>{
    let url = "https://github.com/";
    let res = actix_web::client::Client::default()
        .get(url.to_string())
        .timeout(std::time::Duration::from_secs(2))
        .send()
        .map_err(move |e| {
            println!("error while getting url: {:?}. e: {}", url, e);
            Error::from(e)
        })
    .and_then(|mut rsp| {
        rsp.body().limit(1024 * 1024 * 2).map_err(|e| {
            println!("error while getting payload. e: {}", e);
            Error::from(e)
        })
    });
    res
}

fn proxy_handler() -> impl Future<Item=Response, Error=Error> {
    proxy_service().and_then(|body| HttpResponse::Ok().content_type("text/html").body(body))
    //HttpResponse::Ok().content_type("text/html").body(proxy_service().wait().unwrap())
}

pub fn main() {
    let web_addr = "127.0.0.1:8088";
    println!("Starting foc actix http server at {}",
             &web_addr);
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(web::resource("/").route(web::get().to_async(proxy_handler)))
    })
    .keep_alive(30)
        .bind(web_addr)
        .unwrap()
        .run()
        .unwrap();
}
