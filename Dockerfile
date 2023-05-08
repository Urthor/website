## Use the Rust 1.69.0 Bookworm image as a base
#FROM rust:1.69.0-bookworm
#
## Set the working directory
#WORKDIR /
#
## Set the userto root
#USER root
#
#cargo install dioxus-cli
#
## Copy the project files to the working directory.  Includes release in dist.
#COPY  . .
#
#CMD ["dioxus build --release"]
#
## Exposes port.
#EXPOSE 8000

# Launches my webapp.
FROM nginx:1.24-bullseye

WORKDIR /
USER root

COPY dist .
COPY nginx.conf .

EXPOSE 8000

#ENTRYPOINT ["/usr/sbin/nginx", "-c", "/nginx.conf", ">", "/var/log/nginx/error.log", "2>&1"]
ENTRYPOINT ["/usr/sbin/nginx", "-c", "/nginx.conf"]
#RUN ["nginx", "-c", "/nginx.conf;"]
# docker run -p  192.168.20.3:8000:8000 dioxus-image-1
# TODO: add a SSL certificate for https.