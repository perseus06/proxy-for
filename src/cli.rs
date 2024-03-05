use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Listening ip and port address
    #[clap(short = 'l', long, value_name = "ADDR", default_value = "0.0.0.0:8080")]
    pub listen: String,
    /// Only inspect http(s) traffic whose `{method} {uri}` matches the regex
    #[clap(short = 'f', long, value_name = "REGEX")]
    pub filters: Vec<String>,
    /// Only inspect http(s) traffic whose content-type matches the value
    #[clap(short = 'm', long, value_name = "VALUE")]
    pub mime_filters: Vec<String>,
    /// Forward url
    #[clap(value_name = "URL")]
    pub forward_url: Option<String>,
}
