// drone-activitypub a drone plugin to post to mastodon
// Copyright © 2019 zentauro
//
// This program  is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <http://www.gnu.org/licenses/>.

extern crate elefren;

use elefren::prelude::*;
use elefren::scopes::{Scopes, Write};
use elefren::helpers::{cli, json};

use std::env::var;
use std::error::Error;
use std::alloc::System;

#[global_allocator]
static A: System = System;

fn main() {
    // Obtain the message from the plugin configuration
    let message = var("DRONE_MESSAGE")
        .expect("Failed to retrieve the message to post, please set \"message\"");

    // Mastodon related vars
    let mastodon_url   = var("DRONE_MASTODON_URL")
        .expect("Failed to retrieve the base url of the Mastodon server");
    let mastodon_data  = var("DRONE_MASTODON_DATA")
        .expect("Failed to retrieve the auth data of the mastodon server");

    // Create mastodon client
    // Authentificate with mastodon
    // If we don't have a registered client, register one
    let mastodon = if let Ok(data) = json::from_str(&mastodon_data) {
        Mastodon::from(data)
    } else {
        register(&mastodon_url)
            .expect("First time registration failed")
    };

    // Create the toot and post it
    let result = mastodon.new_status(
        StatusBuilder {
            status: message,
            sensitive: None,
            spoiler_text: None,
            language: None,
            ..Default::default()
        }
    ).expect("Error while communicating with mastodon");

    // Log the reply from mastodon
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
