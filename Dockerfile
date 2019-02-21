FROM zentauro/openssl-glibc:latest
LABEL Name=drone-activitypub Version=0.0.1
ADD ./target/release/drone-activitypub /bin/drone-activitypub
ENTRYPOINT [ "/bin/drone-activitypub" ]
