echo "1. Running a soroban-precview docker container"
previewVersion="8"

echo "Searching for a previous soroban-preview docker container"
containerID=$(docker ps --filter=`name=soroban-preview-${previewVersion}` --all --quiet)
if [[ ${containerID} ]]; then
    echo "Start removing soroban-preview-${previewVersion}  container."
    docker rm --force soroban-preview-${previewVersion}
    echo "Finished removing soroban-preview-${previewVersion} container."
else
    echo "No previous soroban-preview-${previewVersion} container was found"
fi

currentDir=$(pwd)
docker run --volume  ${currentDir}:/workspace \
           --name soroban-preview-8 \
           --interactive \
           --tty \
           -p 8001:8000 \
           --detach \
           --ipc=host \
           --network soroban-network \
           esteblock/soroban-preview:8


echo "2. Running a stellar quickstart docker container"

# Run the stellar quickstart image
docker run --rm -ti \
  --platform linux/amd64 \
  --name stellar \
  --network soroban-network \
  -p 8000:8000 \
  stellar/quickstart:soroban-dev@sha256:81c23da078c90d0ba220f8fc93414d0ea44608adc616988930529c58df278739 \
  $ARGS \
  --enable-soroban-rpc \
  --protocol-version 20 
