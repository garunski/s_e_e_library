use std::path::PathBuf;

use clap::{Parser, Subcommand};
use see_library_catalog_tool::{
    default_dx_public, default_hub_root, default_staging, run_catalog, run_discover_bundles,
    run_harvest, run_package_pages, run_validate, run_verify_docs, smoke_pages_artifact,
};

#[derive(Parser)]
#[command(name = "see-library-catalog")]
struct Cli {
    #[command(subcommand)]
    command: Command,
    #[arg(long, default_value = ".")]
    root: PathBuf,
}

#[derive(Subcommand)]
enum Command {
    /// Read-only validation; rejects stale catalog.json or public mirror.
    Validate,
    /// Regenerate catalog.json from packages and marketplace metadata.
    Catalog,
    /// Copy hub payloads into packages/ and regenerate catalog.json.
    Harvest {
        /// Hub checkout root (default: sibling s_e_e_project of the library root).
        #[arg(long)]
        hub: Option<PathBuf>,
    },
    /// Report bundle ids and member edges from packages/ (read-only unless --write).
    DiscoverBundles {
        #[arg(long)]
        write: bool,
    },
    /// Stage Dioxus output and library payloads into target/pages-artifact.
    PackagePages {
        #[arg(long)]
        dx_public: Option<PathBuf>,
        #[arg(long)]
        staging: Option<PathBuf>,
    },
    /// HTTP smoke checks against a running static server (artifact root URL).
    SmokePages {
        #[arg(long)]
        staging: Option<PathBuf>,
        #[arg(long)]
        base_url: String,
    },
    /// Verify staged documentation HTML, llms.txt coverage, and internal links.
    VerifyDocs {
        #[arg(long)]
        staging: Option<PathBuf>,
    },
}

fn main() {
    let cli = Cli::parse();
    let root = cli.root;
    let result = match cli.command {
        Command::Validate => run_validate(&root),
        Command::Catalog => run_catalog(&root),
        Command::Harvest { hub } => {
            let hub = hub.unwrap_or_else(|| default_hub_root(&root));
            run_harvest(&root, &hub)
        }
        Command::DiscoverBundles { write } => run_discover_bundles(&root, write),
        Command::PackagePages { dx_public, staging } => {
            let dx_public = dx_public.unwrap_or_else(|| default_dx_public(&root));
            let staging = staging.unwrap_or_else(|| default_staging(&root));
            run_package_pages(&root, &dx_public, &staging)
        }
        Command::SmokePages { staging, base_url } => {
            let staging = staging.unwrap_or_else(|| default_staging(&root));
            smoke_pages_artifact(&root, &staging, &base_url)
        }
        Command::VerifyDocs { staging } => {
            let staging = staging.unwrap_or_else(|| default_staging(&root));
            run_verify_docs(&root, &staging)
        }
    };
    if let Err(message) = result {
        eprintln!("error: {message}");
        std::process::exit(1);
    }
}
