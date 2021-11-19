// use clap::App;
// use serde::Serializer;
// use std::process::exit;
use structopt::StructOpt;

/// Représente les arguments en paramètres de ligne de commande
#[derive(Debug, StructOpt)]
pub struct CliArguments {
    /// Commande à exécuter
    pub command: String,

    /// Chemin où trouver les fichiers à analyser
    ///
    #[structopt(parse(from_os_str))]
    pub path: std::path::PathBuf,
}

impl CliArguments {
    pub fn new() -> CliArguments {
        CliArguments::from_args()
    }

    pub fn path(&self) -> &std::path::Path {
        self.path.as_path()
    }
}

#[derive(Debug, StructOpt)]
pub struct CliArguments2 {
    /// Commande à exécuter
    pub command: String,

    /// Chemin où trouver les fichiers à analyser
    ///
    #[structopt(parse(from_os_str))]
    pub path: std::path::PathBuf,

    pub recherche: String,
}

impl CliArguments2 {
    pub fn new() -> CliArguments2 {
        CliArguments2::from_args()
    }

    pub fn path(&self) -> &std::path::Path {
        self.path.as_path()
    }
}
