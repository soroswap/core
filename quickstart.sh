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