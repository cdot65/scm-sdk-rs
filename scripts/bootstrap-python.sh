#!/usr/bin/env bash
# Install CI-only portable Python without changing user profiles or the registry.
set -euo pipefail

: "${CI_UV_VERSION:?Set the reviewed uv version}"
: "${UV_PYTHON_INSTALL_DIR:?Set a job-local Python installation directory}"
: "${UV_PYTHON_BIN_DIR:?Set a job-local Python executable directory}"
: "${GITHUB_PATH:?Run with a runner PATH file}"
: "${SCM_SDK_UV_EXECUTABLE:?Use the exact uv executable path from setup}"

scm_sdk_root="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd -P)"
node "$scm_sdk_root/scripts/uv-integrity.cjs" verify "$SCM_SDK_UV_EXECUTABLE"
scm_sdk_uv_version="$("$SCM_SDK_UV_EXECUTABLE" --version)"
if [[ "$scm_sdk_uv_version" != "uv $CI_UV_VERSION" && "$scm_sdk_uv_version" != "uv $CI_UV_VERSION "* ]]; then
  printf '%s\n' 'The uv executable does not match the reviewed version.' >&2
  exit 1
fi

scm_sdk_python_version="$(cat "$scm_sdk_root/.python-version")"
scm_sdk_python_version="${scm_sdk_python_version%$'\r'}"
if [[ ! "$scm_sdk_python_version" =~ ^3\.[0-9]+\.[0-9]+$ ]]; then
  printf '%s\n' 'Use one exact stable Python 3 version.' >&2
  exit 1
fi

"$SCM_SDK_UV_EXECUTABLE" --no-config --no-cache python install --default --no-registry "$scm_sdk_python_version"
printf '%s\n' "$UV_PYTHON_BIN_DIR" >> "$GITHUB_PATH"
