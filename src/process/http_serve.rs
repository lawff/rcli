use anyhow::Result;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use std::{
    fs,
    net::SocketAddr,
    path::{self, PathBuf},
    sync::Arc,
};
use tower_http::services::ServeDir;
use tracing::{info, warn};

#[derive(Debug)]
struct HttpServeState {
    path: PathBuf,
}

pub async fn process_http_serve(path: PathBuf, port: u16) -> Result<()> {
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("Serving {:?} on {}", path, addr);

    let state = HttpServeState { path: path.clone() };
    let router = Router::new()
        .route("/", get(root))
        .nest_service("/static", ServeDir::new(path))
        .route("/*path", get(file_handler))
        .with_state(Arc::new(state));

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, router).await?;
    Ok(())
}

pub trait PathExector {
    fn execute(&self, base_url: &path::Path) -> (StatusCode, Response);
}

impl PathExector for PathBuf {
    fn execute(&self, base_url: &path::Path) -> (StatusCode, Response) {
        info!("Reading path {:?}", self);
        if self.is_dir() {
            match fs::read_dir(self) {
                Ok(entries) => {
                    let mut content = String::new();
                    content.push_str("<html><body><ul>");
                    for entry in entries.flatten() {
                        let path = entry.path();
                        let name = path.file_name().unwrap().to_string_lossy();
                        content.push_str(&format!(
                            r#"<li><a href="/static/{}">{}</a></li>"#,
                            path.strip_prefix(base_url).unwrap().to_string_lossy(),
                            name
                        ));
                    }
                    content.push_str("</ul></body></html>");
                    (StatusCode::OK, Html(content).into_response())
                }
                Err(e) => {
                    warn!("Failed to read directory: {:?}", e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Failed to read directory: {:?}", e).into_response(),
                    )
                }
            }
        } else {
            match fs::read_to_string(self) {
                Ok(content) => {
                    info!("Read {} bytes", content.len());
                    (StatusCode::OK, Html(content).into_response())
                }
                Err(e) => {
                    warn!("Failed to read file: {:?}", e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Failed to read file: {:?}", e).into_response(),
                    )
                }
            }
        }
    }
}

async fn root(State(state): State<Arc<HttpServeState>>) -> impl IntoResponse {
    state.path.execute(&state.path)
}

async fn file_handler(
    State(state): State<Arc<HttpServeState>>,
    Path(path): Path<String>,
) -> (StatusCode, Response) {
    let p = std::path::Path::new(&state.path).join(path);

    if !p.exists() {
        (
            StatusCode::NOT_FOUND,
            format!("File {:?} not found", p).into_response(),
        )
    } else {
        p.execute(&state.path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_file_handler() {
        let state = Arc::new(HttpServeState {
            path: PathBuf::from("."),
        });
        let res = file_handler(State(state), Path("Cargo.toml".to_string()))
            .await
            .into_response();
        assert_eq!(res.status(), StatusCode::OK);
    }
}
