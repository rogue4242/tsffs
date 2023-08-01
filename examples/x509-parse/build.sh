#!/bin/bash

# Build the HelloWorld.efi module and copy it into the resource directory for the example
# this only needs to be run if you want to modify the source code for the HelloWorld.efi module,
# otherwise, the EFI is included in the source tree for ease of use

SCRIPT_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &>/dev/null && pwd)
IMAGE_NAME="edk2-build-x509-parse"
CONTAINER_UID=$(echo "${RANDOM}" | sha256sum | head -c 8)
CONTAINER_NAME="${IMAGE_NAME}-tmp-${CONTAINER_UID}"

pushd "${SCRIPT_DIR}" || exit 1

cp "${SCRIPT_DIR}/../../include/tsffs.h" "${SCRIPT_DIR}/src/tsffs.h"

docker build -t "${IMAGE_NAME}" -f "Dockerfile" .
docker create --name "${CONTAINER_NAME}" "${IMAGE_NAME}"
docker cp \
    "${CONTAINER_NAME}:/edk2/X509Parse/Build/CryptoPkg/All/DEBUG_GCC5/X64/X509Parse.efi" \
    "${SCRIPT_DIR}/rsrc/X509Parse.efi"
docker rm -f "${CONTAINER_NAME}"
