use clap::{Parser, Subcommand};
#[derive(Debug, Parser)]
#[command(
    name = "klebnz",
    version = "1.0",
    about = "Machine learning classifier for Kleb
       ************************************************
       Gaurav Sablok,
       Email: codeprog@icloud.com
      ************************************************"
)]
pub struct CommandParse {
    /// subcommands for the specific actions
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// build the graph
    SMILES {
        /// path to the file
        filepath: String,
        /// threads for the analysis
        thread: String,
    },
}
