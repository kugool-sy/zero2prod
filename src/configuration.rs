use serde::{Deserialize,  Serialize};


#[derive(Deserialize, Debug)]
pub struct Settings{
    pub application_port: u16,
    pub database: DatabaseSettings,
}

#[derive(Deserialize, Debug)]
pub struct DatabaseSettings{
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError>{
    let settings = config::Config::builder()
        .add_source(config::File::with_name("configuration.yaml"))
        .build()?;
    settings.try_deserialize::<Settings>()
}

impl DatabaseSettings{
    pub fn connection_string(&self) -> String{
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database_name
        )
    }
}