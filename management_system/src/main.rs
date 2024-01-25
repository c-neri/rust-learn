// use std::env::current_dir;

use actix_cors::Cors;
use actix_web::{get, post, App, HttpResponse, HttpServer, Responder, HttpRequest, http::{header::CONTENT_LENGTH}};
use actix_multipart::Multipart;
use futures_util::TryStreamExt as _;
use tokio::fs;
use tokio::io::AsyncWriteExt as _;
use std::path::Path;
use mime::{Mime, IMAGE_PNG, IMAGE_JPEG};
// use serde::Deserialize;
use uuid::Uuid;

// #[derive(Deserialize)]
// struct FileRequest {
//     filename: String,
// }

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

// payload is mutable because implements async Stream
#[post("/upload")]
async fn create_one_file(mut payload:Multipart, req:HttpRequest) -> HttpResponse {
    
    let dir: &str = "./upload/";
    let max_file_size: usize = 10_000_000;
    let max_file_count: usize = 2;
    let allowed: [Mime; 2] = [IMAGE_PNG, IMAGE_JPEG];

    let content_length: usize = match req.headers().get(CONTENT_LENGTH) {
        Some(value) => value.to_str().unwrap_or("0").parse().unwrap(),
        None => 0,
    };

    if content_length == 0 || content_length > max_file_size { 
        return HttpResponse::BadRequest().into()
    };

    let mut counter: usize = 0;
    let mut ids: Vec<String> = Vec::new();

    loop {
        if counter >= max_file_count { break; }
        if let Ok(Some(mut field)) = payload.try_next().await {
            let file_type: Option<&mime::Mime> = field.content_type();

            if file_type.is_none() || !allowed.contains(&file_type.unwrap())  {
                return HttpResponse::BadRequest().body("TypeNotSupportedError")
            }

            let unique_code: String = Uuid::new_v4().to_string();
            let destination: String = format!(
                "{}{}-{}",
                dir,
                unique_code,
                field.content_disposition().get_filename().unwrap()
            );
            log::info!("Destination path: {}", destination);
        
            let mut new_file: fs::File = match fs::File::create(&destination).await {
                Ok(file) => file,
                Err(e) => return HttpResponse::InternalServerError().body(format!("FileCreationError: {}", e)),
            };

            while let Ok(Some(data_chunk)) = field.try_next().await {
                   if let Err(e) = new_file.write_all(&data_chunk).await {
                        return HttpResponse::InternalServerError().body(format!("File write error: {}", e));
                }
            }
            ids.push(unique_code.clone());
        } else { break;}

        counter +=1;
        
    }
    HttpResponse::Ok().json(serde_json::json!({ "ids": ids }))

}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    if !Path::new("./upload").exists() {
        fs::create_dir("./upload").await?;
    }

    HttpServer::new(|| {
            let cors = Cors::permissive();
        App::new()
        .wrap(cors)
            .service(hello)
            .service(create_one_file)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}