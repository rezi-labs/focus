use actix_web::{App, HttpServer, middleware::Logger, web};
use chrono::Utc;
use env_logger::Env;
use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;

mod config;
mod routes;
mod view;

fn main() -> std::io::Result<()> {
    // Check for CLI commands
    let args: Vec<String> = env::args().collect();

    // If there are arguments, create a post
    if args.len() > 1 {
        return create_post(&args);
    }

    // Otherwise start the web server
    start_server()
}

#[actix_web::main]
async fn start_server() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("debug"));

    let c = config::from_env();
    let host = c.host();
    let port = c.port();

    let url = format!("http://{host}:{port}");

    log::info!("Server started at {url}");

    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(c.clone()))
            .service(view::index_route)
            .service(view::about_readme_endpoint)
            .service(view::about::post_route)
            .service(routes::assets::scope())
    });
    server
        .bind((host, port))
        .expect("Could not bind server address")
        .run()
        .await
}

fn create_post(args: &[String]) -> std::io::Result<()> {
    if args.len() < 2 {
        eprintln!("Usage: focus <title>");
        eprintln!("Example: focus \"My First Post\"");
        std::process::exit(1);
    }

    let title = args[1..].join(" ");
    let date = Utc::now();
    let date_str = date.format("%Y-%m-%d").to_string();

    // Create slug from title (lowercase, replace spaces with hyphens)
    let slug = title
        .to_lowercase()
        .chars()
        .map(|c| {
            if c.is_alphanumeric() {
                c
            } else if c.is_whitespace() {
                '-'
            } else {
                '\0'
            }
        })
        .filter(|&c| c != '\0')
        .collect::<String>();

    let filename = format!("{date_str}-{slug}.md");
    let filepath = Path::new("posts").join(&filename);

    // Create posts directory if it doesn't exist
    fs::create_dir_all("posts")?;

    // Check if file already exists
    if filepath.exists() {
        eprintln!("Error: Post already exists at {}", filepath.display());
        std::process::exit(1);
    }

    // Create the markdown file with template
    let template = format!(
        "# {title}\n\
         Write your subtitle here\n\
         ---\n\
         \n\
         Write your post content here.\n"
    );

    let mut file = fs::File::create(&filepath)?;
    file.write_all(template.as_bytes())?;

    println!("Created new post: {}", filepath.display());
    Ok(())
}
