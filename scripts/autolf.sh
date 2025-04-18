#!/bin/bash

directory="${1:-/path/to/your/directory}"

# Check whether the directory is existerence
if [ ! -d "$directory" ];
then
  echo "Directory not found: $directory"
  exit 1
fi

# Search every files.
find "$directory" -type f | while read -r file;
do
    # Convert file via dos2unix
  dos2unix "$file"

    # Check and delete temp file
  tmpFile="${file}.tmp"
  if [ -f "$tmpFile" ];
  then
    rm "$tmpFile"
    echo "Deleted temp file: $tmpFile"
  fi
done

echo "Done."
