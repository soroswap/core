previewHash=$(jq -r '.previewHash' configs.json)
previewVersion=$(echo "$previewHash" | cut -d'@' -f1)

docker exec -it soroban-preview-${previewVersion} bash
