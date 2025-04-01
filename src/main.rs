use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_files::NamedFile;
use chrono::Local;
use std::sync::Mutex;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

// Structure to hold our application state
struct AppState {
    request_log: Mutex<Vec<String>>,
    log_file: Mutex<PathBuf>,
}

// Handler for all HTTP methods
async fn handle_request(req: HttpRequest, body: String, data: web::Data<AppState>) -> impl Responder {
    let method = req.method().to_string();
    let path = req.uri().to_string();
    let headers = format!("{:?}", req.headers());
    let time = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    
    let request_info = format!(
        "Time: {}\nMethod: {}\nPath: {}\nHeaders: {}\nBody: {}\n\n",
        time, method, path, headers, body
    );
    
    println!("Received request:\n{}", request_info);
    
    let mut request_log = data.request_log.lock().unwrap();
    request_log.push(request_info.clone());
    
    let log_path = data.log_file.lock().unwrap().clone();
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(log_path)
        .unwrap();
    
    file.write_all(request_info.as_bytes()).unwrap();
    
    HttpResponse::Ok().body("Request processed successfully")
}

async fn ui_page() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(r#"
            <!DOCTYPE html>
            <html>
            <head>
                <title>Mock Service Request Log</title>
            </head>
            <body>
                <h1>Mock Service Request Log</h1>
                <p>This page displays all requests received by the mock service.</p>
                <a href="/download" class="button">Download Request Log</a>
            </body>
            </html>
        "#)
}

async fn download_log(data: web::Data<AppState>) -> impl Responder {
    let log_path = data.log_file.lock().unwrap().clone();
    
    NamedFile::open(log_path)
        .unwrap()
        .set_content_disposition(
            actix_web::http::header::ContentDisposition {
                disposition: actix_web::http::header::DispositionType::Attachment,
                parameters: vec![actix_web::http::header::DispositionParam::Filename(
                    String::from("request_log")
                )],
            }
        )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let log_file_path = PathBuf::from("request_log");
    File::create(&log_file_path)?;
    
    println!("Starting mock service on http://127.0.0.1:8080");
    println!("UI available at http://127.0.0.1:8080/ui");
    
    let app_state = web::Data::new(AppState {
        request_log: Mutex::new(Vec::new()),
        log_file: Mutex::new(log_file_path),
    });
    
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/ui", web::get().to(ui_page))
            .route("/download", web::get().to(download_log))
            .default_service(web::to(handle_request))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}