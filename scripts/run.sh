previewHash=$(jq -r '.previewHash' preview_version.json)
previewVersion=$(echo "$previewHash" | cut -d'@' -f1)

docker exec -it soroban-preview-${previewVersion} bash
