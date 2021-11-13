use clap::{crate_version, App, Arg, SubCommand};
use config::{Config, ConfigError, Environment, File};
use shellexpand;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub server: Option<String>,
    pub user: Option<String>,
    pub token: Option<String>,
    pub trust: Option<bool>,
    pub debug: Option<bool>,
    pub jobs: Option<CommandJobs>,
}

#[derive(Default, Debug, Deserialize)]
pub struct CommandJobs {
    pub recursive: Option<bool>,
    pub filters: Option<Vec<String>>,
}

impl Settings {
    fn app() -> App<'static, 'static> {
        let a: App = App::new("jestas")
            .version("1.0")
            .author("Trever Shick <trevershick@gmail.com>")
            .about("Gets Jenkins job status")
            .arg(
                Arg::with_name("config")
                    .short("c")
                    .long("config")
                    .value_name("FILE")
                    .help("Configuration file location")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("server")
                    .short("s")
                    .long("server")
                    .help("Jenkins server URL")
                    .value_name("URL")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("user")
                    .short("u")
                    .long("user")
                    .help("Username for authentication (optional)")
                    .value_name("username")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("token")
                    .short("t")
                    .long("token")
                    .help("API token for authentication (optional)")
                    .value_name("token")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("trust")
                    .short("k")
                    .long("trust")
                    .help("Allow self-signed certificates")
                    .takes_value(false),
            )
            .arg(
                Arg::with_name("help")
                    .short("h")
                    .long("help")
                    .takes_value(false),
            )
            .version(crate_version!())
            .subcommand(
                SubCommand::with_name("jobs").arg(
                    Arg::with_name("recursive")
                        .short("r")
                        .long("recursive")
                        .takes_value(false))
                    .arg(
                    Arg::with_name("filters").takes_value(true).multiple(true))
            );
        return a;
    }

    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::default();
        let matches = Settings::app().get_matches();
        let config_file = shellexpand::tilde(matches.value_of("config").unwrap_or("~/.jestas.toml"));
        // Start off by merging in the "default" configuration file
        if std::path::Path::new(config_file.as_ref()).exists() {
            s.merge(File::with_name(&config_file))?;
        }
        // Add in settings from the environment (with a prefix of APP)
        // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
        s.merge(Environment::with_prefix("jestas"))?;

        if let Some(v) = matches.value_of("server") {
            s.set("server", v)?;
        }

        if let Some(m) = matches.subcommand_matches("jobs") {
            s.set("jobs.recursive", m.is_present("recursive"))?;
            if let Some(filters) = m.values_of("filters") {
                s.set("jobs.filters", filters.collect::<Vec<&str>>())?;
            }
        }
        debug!("debug: {:?}", s.get_bool("debug"));
        s.try_into()
    }
}
