//! Simple echo websocket server.
//!
//! Open `http://localhost:8080/` in browser to test.

use std::{path::Path, fs::File, io::Write};

use actix_web::{middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer, Result, Responder};
use actix_web_actors::ws;
use serde::{Serialize, Deserialize};
use serde_json::to_string;

mod server;
use self::server::MyWebSocket;

#[derive(Serialize, Deserialize)]
struct Req {
    name: String,
    version: String
}

#[derive(Serialize)]
struct Res {
    code: i8,
    message: String
}

/// WebSocket handshake and start `MyWebSocket` actor.
async fn echo_ws(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    ws::start(MyWebSocket::new(), &req, stream)
}

async fn update(data: web::Json<Req>) -> Result<impl Responder> {
    let filepath = Path::new("configs/metadata.json");
    let mut file = File::create(filepath).expect("文件创建失败");
    let str = to_string(&data)?;

    file.write_all(str.as_bytes()).expect("写入文件失败");

    let res = Res {
        code: 0,
        message: "创建成功".to_string()
    };

    Ok(web::Json(res))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("starting HTTP server at http://localhost:8081");

    HttpServer::new(|| {
        App::new()
            // websocket route
            .service(web::resource("/service/update").route(web::post().to(update)))
            .service(web::resource("/ws/subscribe").route(web::get().to(echo_ws)))
            // enable logger
            .wrap(middleware::Logger::default())
    })
    .workers(2)
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}