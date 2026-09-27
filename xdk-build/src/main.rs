// Declare modules
mod error;
mod python;
mod typescript;
mod utils;

use crate::error::{BuildError, Result};

use clap::{Parser, Subcommand};
use std::path::{Path, PathBuf};
use xdk_lib::{SdkGeneratorError, log_info, log_warn};
use xdk_openapi::{OpenApi, OpenApiContextGuard, parse_json, parse_json_file, parse_yaml_file};

const LATEST_SPEC_URL: &str = "https://api.x.com/2/openapi.json";

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate a Python SDK from an OpenAPI specification
    Python {
        /// Path to the OpenAPI specification file
        #[arg(short, long)]
        spec: Option<PathBuf>,

        #[arg(short, long)]
        latest: Option<bool>,

        /// Output directory for the generated SDK
        #[arg(short, long, default_value = "xdk/python")]
        output: PathBuf,
    },
    /// Generate a TypeScript SDK from an OpenAPI specification
    #[command(name = "typescript")]
    TypeScript {
        /// Path to the OpenAPI specification file
        #[arg(short, long)]
        spec: Option<PathBuf>,

        #[arg(short, long)]
        latest: Option<bool>,

        /// Output directory for the generated SDK
        #[arg(short, long, default_value = "xdk/typescript")]
        output: PathBuf,
    },
}

fn parse_spec_file(spec_path: &Path) -> Result<OpenApi> {
    let extension = spec_path
        .extension()
        .and_then(|ext| ext.to_str())
        .ok_or_else(|| BuildError::CommandFailed("Invalid file extension".to_string()))?;

    let spec_path_str = spec_path.to_string_lossy();
    match extension {
        "yaml" | "yml" => parse_yaml_file(spec_path_str.as_ref())
            .map_err(|e| SdkGeneratorError::from(e.to_string()).into()),
        "json" => parse_json_file(spec_path_str.as_ref())
            .map_err(|e| SdkGeneratorError::from(e.to_string()).into()),
        _ => Err(BuildError::CommandFailed(format!(
            "Unsupported file extension: {}",
            extension
        ))),
    }
}

async fn fetch_latest_openapi() -> Result<OpenApi> {
    let client = reqwest::Client::new();
    let response =
        client.get(LATEST_SPEC_URL).send().await.map_err(|e| {
            BuildError::CommandFailed(format!("Failed to fetch OpenAPI spec: {}", e))
        })?;

    if !response.status().is_success() {
        return Err(BuildError::CommandFailed(format!(
            "Failed to fetch OpenAPI spec: received HTTP {}",
            response.status()
        )));
    }

    let json_text = response
        .text()
        .await
        .map_err(|e| BuildError::CommandFailed(format!("Failed to read response: {}", e)))?;

    parse_json(&json_text).map_err(|e| SdkGeneratorError::from(e.to_string()).into())
}

async fn load_openapi(spec: Option<PathBuf>, latest: Option<bool>) -> Result<OpenApi> {
    if latest == Some(true) {
        let fallback_path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("latest-openapi.json");

        match fetch_latest_openapi().await {
            Ok(openapi) => return Ok(openapi),
            Err(fetch_error) => {
                log_warn!(
                    "Failed to fetch latest OpenAPI spec ({}). Falling back to local {}.",
                    fetch_error,
                    fallback_path.display()
                );
            }
        }

        return parse_json_file(fallback_path.to_string_lossy().as_ref())
            .map_err(|e| SdkGeneratorError::from(e.to_string()).into());
    }

    let spec_path = spec.ok_or_else(|| {
        BuildError::CommandFailed(
            "A spec path is required when --latest is not set to true.".to_string(),
        )
    })?;
    parse_spec_file(&spec_path)
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Initialize OpenApi context once
    let _guard = OpenApiContextGuard::new();

    let result = match cli.command {
        Commands::Python {
            spec,
            output,
            latest,
        } => {
            let openapi = load_openapi(spec, latest).await?;

            log_info!("Specification parsed successfully.");

            // Call the generate method - `?` handles the Result conversion
            python::generate(&openapi, &output)
        }
        Commands::TypeScript {
            spec,
            output,
            latest,
        } => {
            let openapi = load_openapi(spec, latest).await?;

            log_info!("Specification parsed successfully.");

            // Call the generate method - `?` handles the Result conversion
            typescript::generate(&openapi, &output)
        }
    };

    // Handle the result with better error messaging
    if let Err(ref error) = result {
        use xdk_lib::log_error;
        match error {
            BuildError::IoError(io_error) => match io_error.kind() {
                std::io::ErrorKind::NotFound => {
                    log_error!("Command or file not found. This might be because:");
                    log_error!("  • 'uv' is not installed (install with: pip install uv)");
                    log_error!("  • A required file was not generated properly");
                    log_error!("  • Path does not exist");
                    log_error!("Original error: {}", io_error);
                }
                std::io::ErrorKind::PermissionDenied => {
                    log_error!("Permission denied. Please check file/directory permissions.");
                    log_error!("Original error: {}", io_error);
                }
                _ => {
                    log_error!("File system error occurred: {}", io_error);
                }
            },
            BuildError::CommandFailed(msg) => {
                log_error!("Command execution failed: {}", msg);
            }
            BuildError::FormatterFailed(msg) => {
                log_error!("Code formatting failed: {}", msg);
            }
            BuildError::SdkGenError(msg) => {
                log_error!("SDK generation failed: {}", msg);
            }
        }
        std::process::exit(1);
    }

    result
}
