#[macro_use] extern crate rocket;
use rocket::figment::{providers::{Env, Format, Serialized, Toml}, Figment, Profile};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Config {
    port: u16,
    /* and so on.. */
}

impl Default for Config {
    fn default() -> Config {
        let port_string: String = std::env::var("PORT").unwrap_or("8000".to_string());
        let port: u16 = port_string.parse().unwrap();
        Config {
            port,
        }
    }
}


#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    let figment = Figment::from(rocket::Config::default())
        .merge(Serialized::defaults(Config::default()))
        .merge(Toml::file("App.toml").nested())
        .merge(Env::prefixed("APP_").global())
        .select(Profile::from_env_or("APP_PROFILE", "default"));
    rocket::custom(figment).mount("/", routes![index])
}