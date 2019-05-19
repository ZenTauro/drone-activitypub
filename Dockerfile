FROM zentauro/openssl-glibc:latest
LABEL Name=drone-activitypub Version=v2.0.0
ADD ./target/release/drone-activitypub /bin/drone-activitypub
ENTRYPOINT [ "/bin/drone-activitypub" ]
