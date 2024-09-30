# Use an official Ubuntu base image
FROM ubuntu:22.04
FROM rust:latest

# Set the maintainer label
LABEL maintainer="pericollinas@gmail.com"

# Not install suggested dependencies
RUN echo 'APT::Install-Suggests "0";' >> /etc/apt/apt.conf.d/00-docker && \
    echo 'APT::Install-Recommends "0";' >> /etc/apt/apt.conf.d/00-docker

# Install necessary dependencies
RUN apt-get update && apt-get install -y curl build-essential

# Create and switch to the appruner user
RUN useradd -m -s /bin/bash appruner
USER appruner
WORKDIR /home/appruner

# Copy the application data
COPY --chown=appruner:appruner ./dependencies ./dependencies
COPY --chown=appruner:appruner ./serverdata ./serverdata
COPY --chown=appruner:appruner ./common ./common
COPY --chown=appruner:appruner ./macros ./macros
COPY --chown=appruner:appruner ./server ./server

# Switch back to root to install JDK (requires root privileges)
USER root
RUN apt-get install -y ./dependencies/jdk-21_linux-x64_bin.deb

# Switch back to the appruner user to run the application
USER appruner
WORKDIR /home/appruner/server

# The CMD instruction to run your application
RUN cargo build
CMD ["cargo", "run"]
