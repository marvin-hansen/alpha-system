
1) Pull the RBE image

docker pull --platform=linux/amd64 ghcr.io/marvin-hansen/rbe-custom/rbe:latest

2) Start the docker image

docker run -d -i --platform=linux/amd64 ghcr.io/marvin-hansen/rbe-custom/rbe

3) Connect to the docker image

docker exec -it rbe bash