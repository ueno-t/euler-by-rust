FROM rust:1.88-bullseye

ARG USERNAME=vscode
ARG USER_UID=1000
ARG USER_GID=${USER_UID}

RUN \
    # apt-get
    apt-get update && \
    apt-get -y install git sudo && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/* && \
    # add user
    groupadd --gid ${USER_GID} ${USERNAME} && \
    useradd --uid ${USER_UID} --gid ${USER_GID} -m ${USERNAME} --shell /bin/bash && \
    echo "${USERNAME} ALL=(ALL) NOPASSWD: ALL" > /etc/sudoers.d/${USERNAME} && \
    chmod 0440 /etc/sudoers.d/${USERNAME} && \
    # install rustup
    rustup component add rust-analysis rust-src rustfmt clippy && \
    cargo install cargo-edit cargo-watch

USER ${USERNAME}
