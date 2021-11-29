mod routes;

use rocket::config::Config;
use rocket::log::LogLevel;
use rocket::{catchers, routes, Build, Rocket};
use rocket_slogger::Slogger;
use routes::{always_fail, always_greet, always_thank, not_found};

#[cfg(feature = "terminal")]
fn logger() -> Slogger {
    Slogger::new_terminal_logger()
}

#[cfg(not(feature = "terminal"))]
fn logger() -> Slogger {
    todo!("Re-run this example with `--features terminal`")
}

#[rocket::launch]
async fn rocket() -> Rocket<Build> {
    // fairing built in another function just to ensure
    // that this example runs with the feature enabled
    let fairing = logger();

    let mut config = Config::default();
    config.log_level = LogLevel::Off;

    rocket::custom(config)
        .attach(fairing)
        .mount("/", routes![always_greet, always_thank, always_fail])
        .register("/", catchers![not_found])
}
