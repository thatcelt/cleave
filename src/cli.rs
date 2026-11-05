use clap::{Parser, Subcommand};

use crate::enums::{Format, HttpMethod};

#[derive(Parser, Debug)]
#[command(version)]
#[command(about = "Free and open-source CLI to send and receive requests")]
pub struct Cli {
    /// Enables GUI mode
    #[arg(long)]
    gui: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Initializes a cleave enviroment
    ///
    /// Creates `reqs/` and `cleave.ini`
    Init {
        /// Folder name for cleave project
        #[arg(default_value_t = String::from("."))]
        name: String,

        // Base URL to all endpoints
        #[arg(short, long)]
        base_url: Option<String>,

        /// Header value which will be setted as `Cookie`
        #[arg(short, long)]
        cookie: Option<String>,

        /// Header value which will be setted as `Authorization`
        #[arg(short, long)]
        authorization: Option<String>,

        /// Skips all options from inquire
        #[arg(short)]
        skip: bool,
    },

    /// Parses API spec file into `reqs/` folder
    ///
    /// Gets first openapi.* or swagger.* file without any args
    Parse {
        /// Filename of API spec
        file: Option<String>,
    },

    /// Builds all saved requests from `reqs` to API spec
    Build {
        /// File format for API spec
        #[arg(value_enum, default_value_t = Format::Json)]
        format: Format,
    },

    /// Sends HTTP/HTTPS request
    Http {
        /// API endpoint or full URL
        #[arg(short, long)]
        endpoint: Option<String>,

        #[arg(short, long, value_enum, default_value_t = HttpMethod::Get)]
        method: HttpMethod,

        #[arg(short, long)]
        body: Option<String>,

        #[arg(short = 'H', long)]
        headers: Option<String>,

        #[arg(short, long)]
        proxy: Option<String>,

        /// File path of saved req in cleave `reqs` folder enviroment
        #[arg(long)]
        from_file: Option<String>,

        // If selected `from_file` flag, then opens editor of this request
        #[arg(long)]
        edit: bool,
    },
}
