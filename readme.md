# Serving HLS Files with Actix Web in Rust

1. `Setting up your environment`:

First, make sure you have Rust, Cargo, and ffmpeg installed.

- Install Rust and Cargo from (Rust's official site)[https://www.rust-lang.org/].
- You can typically install `ffmpeg` using a package manager. For instance, on macOS, you'd use `brew install ffmpeg`.

2. `Create a new Rust project`:

```bash
cargo new actix_hls_server
cd actix_hls_server
```

3. `Update Cargo.toml`:

Add the required dependencies:

```toml
[dependencies]
actix-files = "0.6.2"
actix-web = "4"
```

4. `Preparing HLS Content`:

Before serving the content, you need to segment your MP3 file using `ffmpeg`. Run the following command to segment an `input.mp3` file:

```bash
ffmpeg -i input.mp3 -c:a aac -b:a 128k -vn -hls_time 10 -hls_list_size 0 -hls_segment_filename "output%03d.ts" output.m3u8
```

This command will:

- Convert the audio codec to AAC (`-c:a aac`).
- Set the bitrate for the output audio to 128k (`-b:a 128k`).
- Segment the file into chunks with a duration of 10 seconds each (`-hls_time 10`).
- Create an unbounded playlist (`-hls_list_size 0`).
- Name the output segments in the format `output001.ts`, `output002.ts`, etc. (`-hls_segment_filename "output%03d.ts"`).
- Generate an output.m3u8 playlist file.

Move the generated `.m3u8` and `.ts` files to the `hls` directory.

5. `Understanding the Code`:

`Imports`:

```rs
use actix_files as files;
use actix_web::{App, HttpRequest, HttpServer, Result};
```

Here, we're importing the required modules from the `actix-web` and `actix-files` crates.

`HLS Handler`:

```rs
async fn hls_handler(req: HttpRequest) -> Result<files::NamedFile> {
    let path: std::path::PathBuf = req.match_info().query("tail").parse().unwrap();

    // Your custom logic here
    println!("A .m3u8 file was requested!");

    // Now serve the file
    let file_path = format!("./hls/{}", path.display());
    Ok(files::NamedFile::open(file_path)?)
}
```

This is a custom asynchronous handler function that's called when a client requests an HLS file. It reads the path from the request, prints a message, constructs the full path to the requested file, and then serves that file.

`Server Logic`:

```rs
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            actix_web::web::resource("/hls/{tail:.*}").route(actix_web::web::get().to(hls_handler)),
        )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
```

This is the main server logic. The server is set to listen on all available IP addresses (`0.0.0.0`) on port `8080`. It routes GET requests with paths that start with `/hls/` to our custom handler.

6. `Running the server`:

Save the above code in `src/main.rs` and use the following command to run it:

```bash
cargo run
```

7. `Testing the server`:

Once the server is running, you can test it by placing some `.m3u8` files in a folder named `hls` (at the same level as your `Cargo.toml`).

For example, if you have a file named `sample.m3u8` in the `hls` folder, you can access it via a browser or any HTTP client by going to `http://localhost:8080/hls/sample.m3u8`.
