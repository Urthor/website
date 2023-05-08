# Use the Rust 1.69.0 Bookworm image as a base
FROM rust:1.69.0-bookworm

# Set the working directory
WORKDIR /app

# Set the userto root
USER root

# Copy the project files to the working directory.  Includes release in dist.
COPY dist .

# Exposes port.
EXPOSE 8000

# Launches my webapp.
CMD ["dioxus", "serve", "--port", "8000"]

# docker run -p  192.168.20.3:8000:8000 dioxus-image-1
# TODO: add a SSL certificate for https.