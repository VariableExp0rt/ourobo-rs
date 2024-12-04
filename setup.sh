#!/usr/bin/env bash

set -euo pipefail

UBUNTU_RELEASE="24.04"

VM_NAME="local"
VM_ARCH="$(uname -m)"
export VM_CONFIG_TYPE="${LIMA_VM_TYPE:-qemu}"
export VM_CONFIG_CPU="${LIMA_VM_CPU:-4}"
export VM_CONFIG_MEM="${LIMA_VM_MEM:-4GiB}"
export VM_CONFIG_DISK_SZ="${LIMA_VM_DISK_SIZE:-75GiB}"
export VM_URL="${LIMA_VM_ISO_URL:-"https://cloud-images.ubuntu.com/releases/${UBUNTU_RELEASE}/release/ubuntu-${UBUNTU_RELEASE}-server-cloudimg-${VM_ARCH}.img"}"

VM_MOUNT_PATH=$(pwd)
export VM_MOUNT_PATH
VM_CONFIG_ARCH=$([ "${VM_ARCH}" = "arm64" ] && echo "aarch64" || echo "${VM_ARCH}")
export VM_CONFIG_ARCH

ensure_lima() {
    if ! command -v limactl; then
        echo "Could not find \"lima\" executable. Install with:"
        echo ""
        echo "  brew install lima"

        exit 1
    fi
}

destroy_lima_vm() {
    if limactl list "${VM_NAME}" --quiet --log-level error; then
        echo "Lima VM doesn't exist, nothing to do."
    fi

    STATUS=$(limactl list "${VM_NAME}" -f '{{.Status}}')
    if [[ "${STATUS}" = "Running" ]]; then
        limactl stop "${VM_NAME}"
    fi

    limactl delete "${VM_NAME}"
}

ensure_lima_vm() {
    if ! limactl list "${VM_NAME}" --quiet --log-level error; then
        # shellcheck disable=SC2016
        CONFIG_VARS='$VM_CONFIG_CPU $VM_CONFIG_MEM $VM_CONFIG_DISK_SZ $VM_CONFIG_ARCH $VM_CONFIG_TYPE $VM_URL $VM_MOUNT_PATH'
        envsubst "${CONFIG_VARS}" < lima-default.yaml | yq | limactl create --name="${VM_NAME}" -
    fi

    STATUS=$(limactl list "${VM_NAME}" -f '{{.Status}}')
    if [[ "${STATUS}" = "Stopped" ]]; then
        limactl start "${VM_NAME}"
    elif [[ "${STATUS}" = "Running" ]]; then
        echo "Lima VM running!"
    elif [[ "${STATUS}" =~ Broken|Uninitialized|Unknown ]]; then
        echo "Something went wrong during initialisation of the Lima VM :("
        exit 1
    fi
}

main() {
    case $1 in
        up)
            ensure_lima
            ensure_lima_vm
            ;;
        down)
            destroy_lima_vm
            ;;
        *)
            echo -n "Unknown command invocation"
            exit 1
            ;;
    esac
}

main "${@}"
