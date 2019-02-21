# USAGE
This bot will inspect the status of a drone repo and
publish it to a server that speaks activitypub (ie, mastodon)

The first time you use it, it will ask you to authenticate
with the mastodon server, it will ask you to follow some
steps and then you will need to save the data you can use
the following script:

```sh
#!/bin/bash

# Drone variables
export DRONE_BASE_URL=$yoururl
export DRONE_SECRET_TOKEN=$yourtoken
export DRONE_REPO=$reponame
export DRONE_OWNER=$drone_owner

# Mastodon variables
export DRONE_MASTODON_URL=$your_instance
export DRONE_MASTODON_DATA=$leave_empty

# ./target/release/drone-activitypub
docker run \
    -e DRONE_BASE_URL=${DRONE_BASE_URL} \
    -e DRONE_SECRET_TOKEN=${DRONE_SECRET_TOKEN} \
    -e DRONE_REPO=${DRONE_REPO} \
    -e DRONE_OWNER=${DRONE_OWNER} \
    -e DRONE_MASTODON_URL=${DRONE_MASTODON_URL} \
    -e DRONE_MASTODON_DATA=${DRONE_MASTODON_DATA} \
    --rm zentauro/drone-activitypub:0.0.1
```

This is an example of how to use it:
```yml
kind: pipeline
name: mastodon

steps:
- name: message
  image: "zentauro/drone-activitypub:0.0.1"
  settings:
    base_url: $your_drone_server
    secret_token: $your_drone_secret_token
    repo: $the_repo_to_watch
    owner: $repo_owner
    mastodon_url: $mastodon_url
    mastodon_data: $mastodon_auth_data
```
