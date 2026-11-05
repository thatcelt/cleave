use clap::ValueEnum;

#[derive(ValueEnum, Clone, Debug)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
    Patch,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum Format {
    Json,
    Yaml,
}
