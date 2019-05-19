# USAGE
This bot will inspect the status of a drone repo and
publish it to a server that speaks activitypub (ie, mastodon)

The first time you use it, it will ask you to authenticate
with the mastodon server, it will ask you to follow some
steps and then you will need to save the data you can use
the following script:

## Script
```sh
#!/bin/bash

# The message to post:
# See below how to customize the posted message
export DRONE_OWNER=$drone_owner

# Mastodon variables
export DRONE_MASTODON_URL=$your_instance
export DRONE_MASTODON_DATA=$leave_empty

# ./target/release/drone-activitypub
docker run \
    -e DRONE_MESSAGE=${DRONE_MESSAGE} \
    -e DRONE_MASTODON_URL=${DRONE_MASTODON_URL} \
    -e DRONE_MASTODON_DATA=${DRONE_MASTODON_DATA} \
    --rm zentauro/drone-activitypub:v2.0.0
```

## Inside drone
This is an example of how to use it:
```yml
---
kind: pipeline
name: mastodon

steps:
- name: message
  image: "zentauro/drone-activitypub:0.0.1"
  settings:
    message: $message_template
    mastodon_url: $mastodon_url
    mastodon_data: $mastodon_auth_data
```

# Message templates
```yml
message: |
  {{ #success build.status }}
    Build {{ build.number }} pushed by {{ commit.author }} caused by {{ build.event }} at {{ repo.name }} succeded
  {{ else }}
    Build {{ build.number }} pushed by {{ commit.author }} caused by {{ build.event }} at {{ repo.name }} failed
  {{ /success }}
```

The plugin developer docs aren't really clear about what variables are passed to
the plugins, so please take this list with a grain of salt as it is a best effort.

## Template variables
- **repo.owner**: repository owner
- **repo.name**: repository name
- **build.status**: build status _enumeration_
  - success
  - failure
- **build.event**: build event _enumeration_
  - push
  - pull_request
  - tag
  - deployment
- **build.number**: build number
- **build.commit**: git sha for current commit
- **build.branch**: git branch for current commit
- **build.tag**: git tag for current commit
- **build.ref**: git ref for current commit
- **build.author**: git author for current commit
- **build.link**: link the the build results in drone
- **build.started**: unix timestamp for build started
- **build.finished**: unix timestamp for build finished

## Template functions
- **uppercasefirst** converts the first letter of a string to uppercase
- **uppercase** converts a string to uppercase
- **lowercase** converts a string to lowercase. Example {{lowercase build.author}}
- **datetime** converts a unix timestamp to a date time string. Example {{datetime build.started}}
- **success** returns true if the build is successful
- **failure** returns true if the build is failed
- **truncate** returns a truncated string to n characters. Example {{truncate build.sha 8}}
- **urlencode** returns a url encoded string
- **since** returns a duration string between now and the given timestamp. Example {{since build.started}}

## Sources of the variables
- [Telegram bot](http://plugins.drone.io/appleboy/drone-telegram/)
- [Discord bot](http://plugins.drone.io/appleboy/drone-discord/)


# License
The entire contents of this repository are distributed under the
terms of the [GNU Affero General Public License](./LICENSE.md) unless explicitly
stated.

------

drone-activitypub a drone plugin to post to mastodon

Copyright © 2019 zentauro

This program  is free software: you can redistribute it and/or modify
it under the terms of the GNU Affero General Public License as
published by the Free Software Foundation, either version 3 of the
License, or (at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License
along with this program. If not, see <http://www.gnu.org/licenses/>.
