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
    
    // Skip logging for UI-related routes
    if path == "/ui" || path == "/download" || path == "/api" {
        return HttpResponse::Ok().body("Request processed successfully");
    }
    
    let headers = format!("{:?}", req.headers());
    let time = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    
    // Format the request info as a single line with tab separators
    let request_info = format!(
        "Time: {}\tMethod: {}\tPath: {}\tHeaders: {}\tBody: {}\n",
        time, method, path, headers, body
    );
    
    println!("Received request: {}", request_info);
    
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

async fn ui_page(data: web::Data<AppState>) -> impl Responder {
    // Get the current request log to display in the UI
    let request_log = data.request_log.lock().unwrap();
    let log_entries = request_log.join("");
    
    // Create HTML with logs displayed in a pre element for proper formatting
    let html = format!(r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>Mock Service Request Log</title>
            <style>
                body {{ font-family: Arial, sans-serif; margin: 20px; }}
                h1 {{ color: #333; }}
                .button {{ 
                    display: inline-block;
                    padding: 10px 15px;
                    background-color: #4CAF50;
                    color: white;
                    text-decoration: none;
                    border-radius: 4px;
                    margin-right: 10px;
                    margin-bottom: 20px;
                }}
                .refresh-button {{
                    background-color: #2196F3;
                }}
                pre {{ 
                    background-color: #f5f5f5;
                    padding: 15px;
                    border: 1px solid #ddd;
                    border-radius: 4px;
                    overflow-x: auto;
                    white-space: pre-wrap;
                    word-wrap: break-word;
                }}
                .log-title {{ margin-top: 30px; }}
                .button-container {{ margin-bottom: 20px; }}
            </style>
        </head>
        <body>
            <h1>Mock Service Request Log</h1>
            <p>This page displays all requests received by the mock service.</p>
            
            <div class="button-container">
                <a href="/download" class="button">Download Request Log</a>
                <a href="/ui" class="button refresh-button">Refresh</a>
            </div>
            
            <h2 class="log-title">Current Request Log:</h2>
            <pre>{}</pre>
        </body>
        </html>
        "#, log_entries);
    
    HttpResponse::Ok().content_type("text/html").body(html)
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

// New API endpoint that returns the logs as plain text
async fn api_logs(data: web::Data<AppState>) -> impl Responder {
    let request_log = data.request_log.lock().unwrap();
    let log_content = request_log.join("");
    
    HttpResponse::Ok()
        .content_type("text/plain")
        .body(log_content)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let log_file_path = PathBuf::from("request_log");
    File::create(&log_file_path)?;
    
    println!("Starting mock service on http://0.0.0.0:8080");
    println!("UI available at http://localhost:8080/ui");
    println!("API endpoint: http://localhost:8080/api");
    
    let app_state = web::Data::new(AppState {
        request_log: Mutex::new(Vec::new()),
        log_file: Mutex::new(log_file_path),
    });
    
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/ui", web::get().to(ui_page))
            .route("/download", web::get().to(download_log))
            .route("/api", web::get().to(api_logs))
            .default_service(web::to(handle_request))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}