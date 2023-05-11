#!/bin/bash


bash dioxus
bash dioxus build --release
bash npx tailwind -i src/index.css -o ./public/tailwind.css

docker rm d1
docker build -t dioxus-image-1 .
# docker run -p 8000:8000 --name d1 dioxus-image-1

#do docker push hamishpoole/latest
# npx tailwind -i ./src/index.css -o ./public/tailwind.css --watch