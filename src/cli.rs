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
    /// Read a specific stoic passage
    ///
    /// Formats:
    ///   stoic read meditations 4:3       (Book 4, §3)
    ///   stoic read aurelius 4            (Book 4, all)
    ///   stoic read discourses 1:2-5      (Book 1, §2-5)
    ///   stoic read seneca 13             (Letter 13)
    ///   stoic read med 4:3               (abbreviation)
    ///
    /// Work aliases: meditations/med/aurelius/marcus/ma
    ///               discourses/disc/epictetus/epic/ep
    ///               seneca/letters/sen/ml/epistles
    Read {
        /// Stoic reference, e.g. "meditations 4:3" or "seneca 13" or "med 4:3-5"
        #[arg(required = true, num_args = 1..)]
        reference: Vec<String>,
        /// Language code: en (default), ru, fr, de, la, el
        #[arg(long, default_value = "en")]
        lang: String,
    },

    /// Search across all stoic texts for a phrase or keyword
    Search {
        /// Search query (case-insensitive)
        #[arg(required = true, num_args = 1..)]
        query: Vec<String>,
        /// Language to search in: en, ru, fr, de, la, el
        #[arg(long, default_value = "en")]
        lang: String,
    },

    /// Display a random stoic passage (different each run)
    Random {
        /// Language code: en, ru, fr, de, la, el
        #[arg(long, default_value = "en")]
        lang: String,
    },

    /// Show today's stoic passage (same passage all day, changes each day)
    Daily {
        /// Language code: en, ru, fr, de, la, el
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
