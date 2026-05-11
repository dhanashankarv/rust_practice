use std::env::args;
use std::sync::Arc;
use axum::http::StatusCode;
use tokio::sync::{Mutex, MutexGuard};
use tokio::fs::{OpenOptions, File};
use tokio::io::{AsyncReadExt, AsyncWriteExt, AsyncSeekExt};
use std::error::Error;
use tokio::net::TcpListener;
use axum::{
    routing::get,
    routing::post,
    routing::delete,
    extract::Json,
    extract::State,
    Router,
    response::IntoResponse,
};


#[derive(Clone, Debug)]
struct AppState {
    dbfile_handle: Arc<Mutex<File>>,
    dbdata: Arc<Mutex<Vec<String>>>,
}

async fn read_file_to_memory(state: &AppState) -> Result<(), Box<dyn Error>> {
    let mut contents = String::new();
    let mut file: MutexGuard<File> = state.dbfile_handle.lock().await;
    file.seek(std::io::SeekFrom::Start(0)).await?; // Reset file cursor to the beginning
    file.read_to_string(&mut contents).await?;
    let lines: Vec<String> = contents.lines().map(|line| line.to_string()).collect();
    *state.dbdata.lock().await = lines;
    Ok(())
}

async fn root_get(
    State(state): State<AppState>) -> impl IntoResponse {
    let dbdata = state.dbdata.lock().await;
    if dbdata.is_empty() {
        read_file_to_memory(&state).await.unwrap();
    }
    (StatusCode::OK, dbdata.join("\n"))
}

// On a POST request, write the value to the data file
async fn root_post(
    State(state): State<AppState>,
    Json(body): Json<String>) -> impl IntoResponse {
    println!("POST / with body: {}", body);
    let mut res = state.dbfile_handle.lock().await.write_all(body.as_bytes()).await;
    let mut failed: bool = false;
    match res {
        Ok(_) => {
            state.dbdata.lock().await.push(body);
        },
        Err(e) => {
            eprintln!("Failed to write to file: {}", e);
            failed = true;
        }
    };
    if failed {
        (StatusCode::INTERNAL_SERVER_ERROR, "Failed to write to file".to_string())
    } else {
        res = state.dbfile_handle.lock().await.write_all(b"\n").await;
        match res {
            Ok(_) =>
                (StatusCode::OK, "Data added to file and in-memory vector".to_string()),
            Err(e) =>
                (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to write newline to file: {}", e)),
        }
    }
}

async fn root_del(
    State(state): State<AppState>,
    Json(body): Json<String>) -> impl IntoResponse {
    println!("DELETE / with body: {}", body);
    let mut dbdata = state.dbdata.lock().await;
    if let Some(pos) = dbdata.iter().position(|x| x == &body) {
        dbdata.remove(pos);
        // Rewrite the file with the updated data
        let mut file = state.dbfile_handle.lock().await;
        file.set_len(0).await.unwrap(); // Clear the file
        for line in dbdata.iter() {
            file.write_all(line.as_bytes()).await.unwrap();
            file.write_all(b"\n").await.unwrap(); // Add newline after each entry
        }
        (StatusCode::OK, "Data removed from file and in-memory vector")
    } else {
        (StatusCode::NOT_FOUND, "Data not found in memory")
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let fname: String = args().nth(1).unwrap_or_else(|| "database.txt".to_string());

    let dbfile = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&fname)
            .await?;

    let state: AppState = AppState {
        dbfile_handle: Arc::new(Mutex::new(dbfile)),
        dbdata: Arc::new(Mutex::new(Vec::new())),
    };

    read_file_to_memory(&state).await.unwrap();
    let router_root: Router = Router::<AppState>::new()
                                .route("/", get(root_get))
                                .route("/", post(root_post))
                                .route("/", delete(root_del))
                                .with_state(state);

    let listener = TcpListener::bind("127.0.0.1:5555").await.unwrap(); 

    let server = axum::serve(listener, router_root);

    let _res = server.await;
    
    Ok(())
}
