use actix_web::{ web::{self, Data}, HttpRequest, HttpResponse };


struct AppState {
    servers: Vec<String>
}
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
        let data = web::Data::new(AppState {
            servers: self.servers.clone()
        });
        HttpServer::new(move || App::new()
            .default_service(web::to(Self::handler))
            .app_data(data.clone()))
        .bind(("127.0.0.1", self.port))
        .unwrap()
        .run()
        .await
        .unwrap();
    }

    async fn handler(
        req: HttpRequest,
        // Extractor for shared app state
        data: web::Data<AppState>,
        // Extractor to grab request payload as raw bytes 
        bytes: web::Bytes) -> Result<HttpResponse, Error> {
    
        // Grab the address to forward the request to 
        let server = data.servers[0].clone();
        let uri = format!("{}{}", server, req.uri());

        let request_builder = client
            .request(req.method().clone(), uri)
            .headers(req.headers().into())
            .body(bytes);
       
        // Actual request forwarding
        let response = request_builder.send().await?;
        
        // Get response and send it back to client
        let mut response_builder = HttpResponse::build(response.status());
        
        for header in response.headers().iter() {
            response_builder.append_header(h);
        }

        let body = response.bytes().await?;
        Ok(response_builder.body(body))
        HttpResponse::Ok().body("hello!")
    }
}
fn main() {
    println!("Hello, world!");
}

