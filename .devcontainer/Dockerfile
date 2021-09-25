# See here for image contents: https://github.com/microsoft/vscode-dev-containers/tree/v0.231.6/containers/rust/.devcontainer/base.Dockerfile

# [Choice] Debian OS version (use bullseye on local arm64/Apple Silicon): buster, bullseye
ARG VARIANT="buster"
FROM mcr.microsoft.com/vscode/devcontainers/rust:0-${VARIANT}
RUN rustup target add thumbv6m-none-eabi
RUN cargo install cargo-watch
# RUN cargo install elf2uf2-rs --locked
# RUN cargo install probe-run
# RUN cargo install flip-link
# [Optional] Uncomment this section to install additional packages.
# RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
#     && apt-get -y install --no-install-recommends <your-package-list-here>
