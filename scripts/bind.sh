#! /bin/sh

REPO_ROOT="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
REPO_ROOT="$(dirname "$REPO_ROOT")"
CHAINLIST_TOML="${REPO_ROOT}/etc/superchain-registry/chainList.toml"
CONFIGS_TOML="${REPO_ROOT}/etc/superchain-registry/bindings/rust-bindings/etc/configs.toml"
ALT_CONFIGS_TOML="${REPO_ROOT}/etc/superchain-registry/configs.toml"

# Attempt to copy over the chainList.toml file to crates/registry/etc/chainList.toml
if [ -f "${CHAINLIST_TOML}" ]; then
    cp "${CHAINLIST_TOML}" "${REPO_ROOT}/crates/registry/etc/chainList.toml"
else
    echo "[ERROR] ${CHAINLIST_TOML} does not exist"
    exit 1
fi

# Attempt to copy over the configs.toml file to crates/registry/etc/configs.toml
if [ -f "${CONFIGS_TOML}" ]; then
    cp "${CONFIGS_TOML}" "${REPO_ROOT}/crates/registry/etc/configs.toml"
else
    echo "[WARN] ${CONFIGS_TOML} does not exist"
    echo "[INFO] Attempting to copy configs.toml file from the repo root"
  # Attempt to copy over the configs.toml file to crates/registry/etc/configs.toml
  if [ -f "${ALT_CONFIGS_TOML}" ]; then
      cp "${ALT_CONFIGS_TOML}" "${REPO_ROOT}/crates/registry/etc/configs.toml"
  else
      echo "[ERROR] ${ALT_CONFIGS_TOML} does not exist"
      exit 1
  fi
fi
