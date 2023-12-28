#!/bin/bash

# Set the current directory
currentDir=$(pwd)

# Set the name, image and version for the Docker container
containerName=soroswapCoreApi
imageName=node
versionTag=18.18.2

# Display the command being executed
echo "Command: $1"

# Check if there is a previous Docker container with the same name
echo "Searching for a previous docker container"
containerID=$(docker ps --filter="name=${containerName}" --all --quiet)
if [[ ${containerID} ]]; then
    echo "Start removing container."
    # Remove the previous Docker container
    docker rm --force ${containerName}
    echo "Finished removing container."
else
    echo "No previous container was found"
fi

# Run a new Docker container
docker run --volume ${currentDir}/:/workspace \
           --name ${containerName} \
           --interactive \
           --publish 8010:8010 \
           --workdir="/workspace" \
           --tty \
           --detach \
           --publish-all \
           --network soroban-network \
           ${imageName}:${versionTag}

# Connect to bash on Docker container
# docker exec --tty --interactive $containerName bash

# Install dependencies
docker exec $containerName yarn
# Launch server
docker exec $containerName node /workspace/scripts/api/server.js