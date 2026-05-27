use std::env::args;
use tree_parser::{Language, detect_language_by_extension, parse_file};

/*
use tokio::fs::File;
use tokio::io::AsyncReadExt;
async fn read_file(file_path: std::path::PathBuf) -> Result<String, Box<dyn std::error::Error>> {
    let mut file = File::open(file_path).await?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;
    Ok(contents)
}

fn find_language(file_path: &str) -> Option<Language> {
    if let Some(extension) = std::path::Path::new(file_path).extension() {
        match extension.to_str().unwrap_or_default() {
            "rs" => Some(Language::Rust),
            "js" => Some(Language::JavaScript),
            "py" => Some(Language::Python),
            "c" => Some(Language::C),
            "cpp" => Some(Language::Cpp),
            "java" => Some(Language::Java),
            _ => None,
        }
    } else {
        None
    }
}
 */

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli_args = args().collect::<Vec<String>>();
    if cli_args.len() < 2 {
        return Err(format!("Usage: {} <file_path>", cli_args[0]).into());
    }
    let file_path = &cli_args[1];
    let language = detect_language_by_extension(file_path).unwrap_or_else(|| {
        eprintln!("Could not determine language from file extension. Defaulting to Rust.");
        Language::Rust
    });
    let tree = parse_file(&file_path, language).await?;
    println!("File Details: Name: {} {:?}", tree.file_path, tree.language);
    for construct in tree.constructs.iter() {
        println!(
            "Construct: {} \"{}\" at line {} - {}",
            construct.node_type,
            construct.name.as_ref().map_or("None", |s| s),
            construct.start_line,
            construct.end_line
        );
    }
    Ok(())
}
