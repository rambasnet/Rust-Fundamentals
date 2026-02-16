# Use the official Rust image as a builder
FROM rust:latest

RUN apt update \
  && apt install -y \
  build-essential sqlite3 time curl git nano \
  net-tools iputils-ping iproute2 sudo gdb less \
  && apt clean

RUN apt install -y python3 \
  python3-pip python3-venv

RUN rustup component add rust-src

RUN python3 -m venv /opt/venv
ENV PATH="/opt/venv/bin:$PATH"

RUN pip install --upgrade pip
RUN pip install jupyterlab ipykernel

RUN python3 -m ipykernel install --user --name jupyter --display-name "Python 3.13"

RUN cargo install evcxr_jupyter

RUN evcxr_jupyter --install

# Install zsh - use "Bira" theme with some customization. 
RUN sh -c "$(wget -O- https://github.com/deluan/zsh-in-docker/releases/download/v1.1.5/zsh-in-docker.sh)" -- \
  -t bira \
  -p git \
  -p ssh-agent \
  -p https://github.com/zsh-users/zsh-autosuggestions \
  -p https://github.com/zsh-users/zsh-completions
