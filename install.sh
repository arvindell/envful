#!/usr/bin/env bash

# shellcheck disable=SC2039

# Options
#
#   -V, --verbose
#     Enable verbose output for the installer
#
#   -f, -y, --force, --yes
#     Skip the confirmation prompt during installation
#
#   -p, --platform
#     Override the platform identified by the installer
#
#   -b, --bin-dir
#     Override the bin installation directory
#
#   -B, --base-url
#     Override the base URL used for downloading releases

set -eu
printf '\n'

BOLD="$(tput bold 2>/dev/null || printf '')"
GREY="$(tput setaf 0 2>/dev/null || printf '')"
UNDERLINE="$(tput smul 2>/dev/null || printf '')"
RED="$(tput setaf 1 2>/dev/null || printf '')"
GREEN="$(tput setaf 2 2>/dev/null || printf '')"
YELLOW="$(tput setaf 3 2>/dev/null || printf '')"
BLUE="$(tput setaf 4 2>/dev/null || printf '')"
MAGENTA="$(tput setaf 5 2>/dev/null || printf '')"
NO_COLOR="$(tput sgr0 2>/dev/null || printf '')"

APP_NAME="envful"
APP_AUTHOR="arvindell"

SUPPORTED_TARGETS="macos \
                  linux \
                  win64"

info() {
  printf '%s\n' "${BOLD}${GREY}>${NO_COLOR} $*"
}

warn() {
  printf '%s\n' "${YELLOW}! $*${NO_COLOR}"
}

error() {
  printf '%s\n' "${RED}x $*${NO_COLOR}" >&2
}

completed() {
  printf '%s\n' "${GREEN}✓${NO_COLOR} $*"
}

has() {
  command -v "$1" 1>/dev/null 2>&1
}

# Gets path to a temporary file, even if
get_tmpfile() {
  local suffix
  suffix="$1"
  if has mktemp; then
    printf "%s.%s" "$(mktemp)" "${suffix}"
  else
    # No really good options here--let's pick a default + hope
    printf "/tmp/${APP_NAME}.%s" "${suffix}"
  fi
}

get_tmpdir() {
  if has mktemp; then
    printf "%s" "$(mktemp -d)"
  else
    # No really good options here--let's pick a default + hope
    printf "/tmp/%s" "${APP_NAME}"
  fi
}

# Test if a location is writeable by trying to write to it. Windows does not let
# you test writeability other than by writing: https://stackoverflow.com/q/1999988
test_writeable() {
  local path
  path="${1:-}/test.txt"
  if touch "${path}" 2>/dev/null; then
    rm "${path}"
    return 0
  else
    return 1
  fi
}

download() {
  file="$1"
  url="$2"

  if has curl; then
    cmd="curl --fail --silent --location --output $file $url"
  elif has wget; then
    cmd="wget --quiet --output-document=$file $url"
  elif has fetch; then
    cmd="fetch --quiet --output=$file $url"
  else
    error "No HTTP download program (curl, wget, fetch) found, exiting…"
    return 1
  fi

  $cmd && return 0 || rc=$?

  error "Command failed (exit code $rc): ${BLUE}${cmd}${NO_COLOR}"
  printf "\n" >&2
  info "This is likely due to ${APP_NAME} not yet supporting your configuration."
  info "If you would like to see a build for your configuration,"
  info "please create an issue requesting a build for ${MAGENTA}${TARGET}${NO_COLOR}:"
  info "${BOLD}${UNDERLINE}https://github.com/${APP_AUTHOR}/${APP_NAME}/issues/new/${NO_COLOR}"
  return $rc
}

unpack() {
  local archive=$1
  local untar_dir=$2

  case "$archive" in
    *.tar.gz)
      flags=$(test -n "${VERBOSE-}" && echo "-xzvf" || echo "-xzf")
      tar "${flags}" "${archive}" -C "${untar_dir}"
      cp -r "${untar_dir}"/*/* "${untar_dir}/"
      return 0
      ;;
    *.zip)
      flags=$(test -z "${VERBOSE-}" && echo "-qq" || echo "")
      UNZIP="${flags}" unzip "${archive}" -d "${untar_dir}"
      cp -r "${untar_dir}"/*/* "${untar_dir}/"
      return 0
      ;;
  esac

  error "Unknown package extension."
  printf "\n"
  info "This almost certainly results from a bug in this script--please file a"
  info "bug report at https://github.com/${APP_AUTHOR}/${APP_NAME}/issues"
  return 1
}

setup_package() {
  local untar_dir="$1"
  local sudo="${2-}"
  local bin_dir=$3

  # add main executable to $PATH
  if [ -f "${untar_dir}/${APP_NAME}.exe" ]
    # we are on windows, so we copy the exe executable
    then eval "${sudo}" cp "${untar_dir}/${APP_NAME}.exe" "${bin_dir}/"
    # we are on linux, so we copy the linux executable
    else eval "${sudo}" cp "${untar_dir}/${APP_NAME}" "${bin_dir}/"
  fi
}

elevate_priv() {
  if ! has sudo; then
    error 'Could not find the command "sudo", needed to get permissions for install.'
    info "If you are on Windows, please run your shell as an administrator, then"
    info "rerun this script. Otherwise, please run this script as root, or install"
    info "sudo."
    exit 1
  fi
  if ! sudo -v; then
    error "Superuser not granted, aborting installation"
    exit 1
  fi
}

install() {
  local msg
  local sudo
  local archive
  local ext="$1"

  if test_writeable "${BIN_DIR}"; then
    sudo=""
    msg="Installing ${APP_NAME}, please wait…"
  else
    warn "Escalated permissions are required to install to ${BIN_DIR}"
    elevate_priv
    sudo="sudo"
    msg="Installing ${APP_NAME} as root, please wait…"
  fi
  info "$msg"

  archive=$(get_tmpfile "$ext")

  untar_dir=$(get_tmpdir)

  # download to the temp file
  download "${archive}" "${URL}"

  # unpack the temp file
  unpack "${archive}" "${untar_dir}"

  # setup the package (man pages, docs, and the main executable)
  setup_package "${untar_dir}" "${sudo}" "${BIN_DIR}"
}

# Currently supporting:
#   - win (Git Bash)
#   - darwin
#   - linux
#   - linux_musl (Alpine)
#   - freebsd
detect_platform() {
  local platform
  platform="$(uname -s | tr '[:upper:]' '[:lower:]')"

  case "${platform}" in
    msys_nt*) platform="win64" ;;
    cygwin_nt*) platform="win64";;
    # mingw is Git-Bash
    mingw*) platform="win64" ;;
    # use the statically compiled musl bins on linux to avoid linking issues.
    darwin) platform="macos" ;;
    linux) platform="linux" ;;
    freebsd) platform="linux" ;;
  esac

  printf '%s' "${platform}"
}

detect_version() {
  # adapted from https://gist.github.com/lukechilds/a83e1d7127b78fef38c2914c4ececc3c
  curl --silent "https://api.github.com/repos/${APP_AUTHOR}/${APP_NAME}/releases/latest" |  # Get latest release from GitHub api
    grep '"tag_name":' |                                                                    # Get tag line
    sed -E 's/.*"([^"]+)".*/\1/'                                                            # Pluck JSON value
}

confirm() {
  if [ -z "${FORCE-}" ]; then
    printf "%s " "${MAGENTA}?${NO_COLOR} $* ${BOLD}[y/N]${NO_COLOR}"
    set +e
    read -r yn </dev/tty
    rc=$?
    set -e
    if [ $rc -ne 0 ]; then
      error "Error reading from prompt (please re-run with the '--yes' option)"
      exit 1
    fi
    if [ "$yn" != "y" ] && [ "$yn" != "yes" ]; then
      error 'Aborting (please answer "yes" to continue)'
      exit 1
    fi
  fi
}

check_bin_dir() {
  local bin_dir="$1"

  if [ ! -d "$BIN_DIR" ]; then
    error "Installation location $BIN_DIR does not appear to be a directory"
    info "Make sure the location exists and is a directory, then try again."
    exit 1
  fi

  # https://stackoverflow.com/a/11655875
  local good
  good=$(
    IFS=:
    for path in $PATH; do
      if [ "${path}" = "${bin_dir}" ]; then
        printf 1
        break
      fi
    done
  )

  if [ "${good}" != "1" ]; then
    warn "Bin directory ${bin_dir} is not in your \$PATH"
  fi
}

is_build_available() {
  local platform="$1"
  local target="$2"

  local good

  good=$(
    IFS=" "
    for t in $SUPPORTED_TARGETS; do
      if [ "${t}" = "${target}" ]; then
        printf 1
        break
      fi
    done
  )

  if [ "${good}" != "1" ]; then
    error "Builds for ${platform} are not yet available for ${APP_NAME}"
    printf "\n" >&2
    info "If you would like to see a build for your configuration,"
    info "please create an issue requesting a build for ${MAGENTA}${target}${NO_COLOR}:"
    info "${BOLD}${UNDERLINE}https://github.com/${APP_AUTHOR}/${APP_NAME}/issues/new/${NO_COLOR}"
    printf "\n"
    exit 1
  fi
}

# defaults
if [ -z "${PLATFORM-}" ]; then
  PLATFORM="$(detect_platform)"
fi

if [ -z "${VERSION-}" ]; then
  VERSION="$(detect_version)"
fi

if [ -z "${BIN_DIR-}" ]; then
  BIN_DIR=/usr/local/bin
  if [ ! -d "$BIN_DIR" ]; then
    BIN_DIR=/usr/bin
  fi
fi

if [ -z "${BASE_URL-}" ]; then
  BASE_URL="https://github.com/${APP_AUTHOR}/${APP_NAME}/releases"
fi

# parse argv variables
while [ "$#" -gt 0 ]; do
  case "$1" in
  -p | --platform)
    PLATFORM="$2"
    shift 2
    ;;
  -b | --bin-dir)
    BIN_DIR="$2"
    shift 2
    ;;
  -B | --base-url)
    BASE_URL="$2"
    shift 2
    ;;

  -V | --verbose)
    VERBOSE=1
    shift 1
    ;;
  -f | -y | --force | --yes)
    FORCE=1
    shift 1
    ;;

  -p=* | --platform=*)
    PLATFORM="${1#*=}"
    shift 1
    ;;
  -b=* | --bin-dir=*)
    BIN_DIR="${1#*=}"
    shift 1
    ;;
  -B=* | --base-url=*)
    BASE_URL="${1#*=}"
    shift 1
    ;;
  -V=* | --verbose=*)
    VERBOSE="${1#*=}"
    shift 1
    ;;
  -f=* | -y=* | --force=* | --yes=*)
    FORCE="${1#*=}"
    shift 1
    ;;

  *)
    error "Unknown option: $1"
    exit 1
    ;;
  esac
done

TARGET="${PLATFORM}"

is_build_available "${PLATFORM}" "${TARGET}"

TARGET="${VERSION}-${TARGET}"

printf "  %s\n" "${UNDERLINE}Configuration${NO_COLOR}"
info "${BOLD}Bin directory${NO_COLOR}: ${GREEN}${BIN_DIR}${NO_COLOR}"
info "${BOLD}Platform${NO_COLOR}:      ${GREEN}${PLATFORM}${NO_COLOR}"
info "${BOLD}Version${NO_COLOR}:       ${GREEN}${VERSION}${NO_COLOR}"

# non-empty VERBOSE enables verbose untarring
if [ -n "${VERBOSE-}" ]; then
  VERBOSE=v
  info "${BOLD}Verbose${NO_COLOR}: yes"
else
  VERBOSE=
fi

printf '\n'

EXT=tar.gz
if [ "${PLATFORM}" = "pc-windows-msvc" ]; then
  EXT=zip
fi

URL="${BASE_URL}/latest/download/${APP_NAME}-${TARGET}.${EXT}"
info "Tarball URL: ${UNDERLINE}${BLUE}${URL}${NO_COLOR}"
confirm "Install ${APP_NAME} ${GREEN}latest${NO_COLOR} to ${BOLD}${GREEN}${BIN_DIR}${NO_COLOR}?"
check_bin_dir "${BIN_DIR}"

install "${EXT}"
completed "${APP_NAME} installed"

printf '\n'
info "Run the command displayed below...

  ${BOLD}${MAGENTA}${APP_NAME}${NO_COLOR}
"