use structopt::StructOpt;

/// Représente les arguments en paramètres de ligne de commande
#[derive(Debug)]
#[derive(StructOpt)]
pub struct CliArguments {
    /// S to scan a folder
    /// R to read a json file
    /// SS so search the 
    command: String,

    /// Chemin où trouver les fichiers à analyser
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,

    search: Option<String>,
}

impl CliArguments {
    pub fn new() -> CliArguments {
        CliArguments::from_args()
    }

    pub fn path(&self) -> &std::path::Path {
        self.path.as_path()
    }

    pub fn command(&self) -> String {
        self.command.to_string()
    }

    pub fn search(&self) -> String {
        match &self.search {
            Some(a) => a.to_string(),
            None => panic!("No pattern provided for search")
        }
    }
}
