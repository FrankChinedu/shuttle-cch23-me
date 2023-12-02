use actix_web::{
    get,
    web::{Path, ServiceConfig},
    HttpRequest, HttpResponse,
};
use shuttle_actix_web::ShuttleActixWeb;

#[get("/")]
async fn hello_world() -> &'static str {
    "Hello World!"
}
#[get("/-1/error")]
async fn handle_error() -> HttpResponse {
    HttpResponse::InternalServerError().into()
}

#[derive(Debug, serde::Deserialize)]
struct XorPath {
    nums: Vec<u32>,
}

#[get("/1/{nums:.*}")]
async fn xor_operation(req: HttpRequest) -> String {
    let res = req
        .match_info()
        .query("nums")
        .split('/')
        .map(|x| x.parse::<i32>().expect("msg"))
        .fold(0, |acc, x| acc ^ x)
        .pow(3);

    format!("{res:?}")
}

#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(hello_world)
            .service(handle_error)
            .service(xor_operation);
    };

    Ok(config.into())
}
