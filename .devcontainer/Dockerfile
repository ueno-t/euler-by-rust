FROM rust:1.88-bullseye

ARG username=vscode
ARG useruid=1000
ARG usergid=${useruid}

RUN \
    # apt-get
    apt-get update && \
    apt-get -y install git sudo && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/* && \
    # add user
    groupadd --gid ${usergid} ${username} && \
    useradd --uid ${useruid} --gid ${usergid} -m ${username} --shell /bin/bash && \
    echo "${username} ALL=(ALL) NOPASSWD: ALL" > /etc/sudoers.d/${username} && \
    chmod 0440 /etc/sudoers.d/${username} && \
    # install rustup
    rustup component add rust-analysis rust-src rustfmt clippy && \
    cargo install cargo-edit cargo-watch

USER ${username}