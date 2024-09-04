#! /bin/sh

REPO_ROOT="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
REPO_ROOT="$(dirname "$REPO_ROOT")"
CHAINLIST_JSON="${REPO_ROOT}/etc/superchain-registry/chainList.json"
CONFIGS_JSON="${REPO_ROOT}/etc/superchain-registry/bindings/rust-bindings/etc/configs.json"
ALT_CONFIGS_JSON="${REPO_ROOT}/etc/superchain-registry/superchain/configs/configs.json"

# Attempt to copy over the chainList.json file to crates/registry/etc/chainList.json
if [ -f "${CHAINLIST_JSON}" ]; then
    cp "${CHAINLIST_JSON}" "${REPO_ROOT}/crates/registry/etc/chainList.json"
else
    echo "[ERROR] ${CHAINLIST_JSON} does not exist"
    exit 1
fi

# Attempt to copy over the configs.json file to crates/registry/etc/configs.json
if [ -f "${CONFIGS_JSON}" ]; then
    cp "${CONFIGS_JSON}" "${REPO_ROOT}/crates/registry/etc/configs.json"
else
    echo "[WARN] ${CONFIGS_JSON} does not exist"
    echo "[INFO] Attempting to copy configs.json file from the repo root"
  # Attempt to copy over the configs.json file to crates/registry/etc/configs.json
  if [ -f "${ALT_CONFIGS_JSON}" ]; then
      cp "${ALT_CONFIGS_JSON}" "${REPO_ROOT}/crates/registry/etc/configs.json"
  else
      echo "[ERROR] ${ALT_CONFIGS_JSON} does not exist"
      exit 1
  fi
fi
