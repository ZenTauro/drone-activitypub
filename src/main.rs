extern crate elefren;
extern crate rusty_drone;

use rusty_drone::structs::*;
use rusty_drone::Client as DrClie;

use elefren::prelude::*;
use elefren::scopes::{Scopes, Write};
use elefren::helpers::{cli, json};

use std::env::var;
use std::error::Error;

fn main() {
    // Drone related vars
    let token          = var("DRONE_SECRET_TOKEN")
        .expect("Failed to retrieve DRONE_TOKEN");
    let drone_base_url = var("DRONE_BASE_URL")
        .expect("Failed to retrieve the base url of the Drone server");
    let repo           = var("DRONE_REPO")
       .expect("Failed to retrieve the repo to watch");
    let owner          = var("DRONE_OWNER")
        .expect("Failed to retrieve the repo owner");

    // Mastodon related vars
    let mastodon_url   = var("DRONE_MASTODON_URL")
        .expect("Failed to retrieve the base url of the Mastodon server");
    let mastodon_data  = var("DRONE_MASTODON_DATA")
        .expect("Failed to retrieve the auth data of the mastodon server");

    // Create mastodon client
    // Authentificate with mastodon
    let mastodon = if let Ok(data) = json::from_str(&mastodon_data) {
        Mastodon::from(data)
    } else {
        register(&mastodon_url)
            .expect("First time registration failed")
    };

    // Create Drone client
    let drone_client = DrClie::new(token, drone_base_url);

    // Get response from Drone
    let response = drone_client.get_build_list(
        &owner,
        &repo
    ).expect("Error while communicating with the drone server");

    // Create the toot and post it
    let result = mastodon.new_status(
        StatusBuilder{
            status: format!(
                "{} job: \"{}\" by {} triggered by {}\nrepo: {}",
                match &response[0].status {
                    Status::Failure => "Failed",
                    Status::Success => "Successful"
                },
                response[0].message,
                response[0].author,
                response[0].event,
                repo
            ),
            sensitive: None,
            spoiler_text: None,
            language: None,
            ..Default::default()
        }
    ).expect("Error while communicating with mastodon");

    println!("Data from mastodon: {}",
             result.content
    );
}

fn register(mastodon_url: &String) -> Result<Mastodon, Box<Error>> {
    let registration = Registration::new(format!("https://{}", mastodon_url))
        .client_name("DroneBot")
        .scopes(Scopes::write(Write::Statuses))
        .build()?;

    let mastodon = cli::authenticate(registration)?;

    println!("This is your MASTODON_DATA\n{}",
        json::to_string(&*mastodon)?);

    Ok(mastodon)
}
