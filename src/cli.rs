use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "stoic", about = "A beautiful Stoic philosophy TUI — read Aurelius, Seneca & Epictetus in your terminal")]
#[command(version, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Skip the startup banner animation
    #[arg(long, global = true)]
    pub no_banner: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Read a stoic passage (e.g. "meditations 4:3", "seneca 13", "discourses 1:2")
    Read {
        /// Stoic reference (e.g. "aurelius 4:3", "seneca 13")
        #[arg(required = true, num_args = 1..)]
        reference: Vec<String>,
        /// Optional language code (en, ru, fr, de, la, el)
        #[arg(long, default_value = "en")]
        lang: String,
    },

    /// Search all stoic texts for a phrase or keyword
    Search {
        /// Search query
        #[arg(required = true, num_args = 1..)]
        query: Vec<String>,
        /// Optional language code (en, ru, fr, de, la, el)
        #[arg(long, default_value = "en")]
        lang: String,
    },

    /// Display a random stoic passage
    Random {
        /// Optional language code (en, ru, fr, de, la, el)
        #[arg(long, default_value = "en")]
        lang: String,
    },

    /// Show today's daily stoic passage
    Daily {
        /// Optional language code (en, ru, fr, de, la, el)
        #[arg(long, default_value = "en")]
        lang: String,
    },

    /// Replay the startup animation
    Intro,

    /// Update stoic-cli to the latest version
    Update {
        /// Check for updates without installing
        #[arg(long)]
        check: bool,
    },
}
