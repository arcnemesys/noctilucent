use actix_web::{ HttpRequest, HttpResponse };

struct LoadBalancer {
    port: u16,
    servers: Vec<String>
}

impl LoadBalancer {
    pub fn new(port: u16, servers: Vec<String>) -> Self {
        LoadBalancer { port, servers}
    }

    pub fn uri(&self) -> String {
        format!("http://127.0.0.1:{}", self.port)
    }

    pub async fn run(&self) {
        HttpServer::new(move || App::new()
            .default_service(web::to(handler)))
        .bind(("127.0.0.1", self.port))
        .unwrap()
        .run()
        .await
        .unwrap();
    }

    async fn handler(req: HttpRequest) -> HttpResponse {
        HttpResponse::Ok().body("hello!")
    }
}
fn main() {
    println!("Hello, world!");
}
