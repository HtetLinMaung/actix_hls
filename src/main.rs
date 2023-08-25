use actix_files as files;
use actix_web::{App, HttpRequest, HttpServer, Result};

// Custom handler for the HLS path
async fn hls_handler(req: HttpRequest) -> Result<files::NamedFile> {
    let path: std::path::PathBuf = req.match_info().query("tail").parse().unwrap();

    // Your custom logic here
    println!("A .m3u8 file was requested!");

    // Now serve the file
    let file_path = format!("./hls/{}", path.display());
    Ok(files::NamedFile::open(file_path)?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            actix_web::web::resource("/hls/{tail:.*}").route(actix_web::web::get().to(hls_handler)),
        )
        // .service(files::Files::new("/hls", "./hls").show_files_listing())
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
