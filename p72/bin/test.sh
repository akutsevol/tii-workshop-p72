#!/bin/bash

PROJECT="openssl"

INPUT_FILE="input.txt"
BIN_FILE="encrypted.bin"
DECRYPTED_FILE="decrypted.txt"

generate_key() {
  local OPEN_SSL_APP=$(which openssl)
  local UUIDGEN_APP=$(which uuidgen)
  local HEXDUMP_APP=$(which hexdump)

  local KEY_LENGTH=32
  local KEY=""

  if [ -f "$OPEN_SSL_APP" ]; then
      KEY=$($OPEN_SSL_APP rand -hex $KEY_LENGTH)
  elif [ -f "$UUIDGEN_APP" ]; then
      KEY=$($UUIDGEN_APP | tr -d '-')
      KEY="$KEY$($UUIDGEN_APP | tr -d '-')"
  elif [ -f "$HEXDUMP_APP" ]; then
      KEY=$($HEXDUMP_APP -vn32 -e'4/4 "%08X" 1 "\n"' /dev/urandom | tr -d '\n')
  fi

  echo "$KEY"
}

build_target() {
  # check we are not in 'bin' folder or change it
  local CUR_DIR=$(pwd)
  local ROOT_DIR=$(dirname $(realpath $(find $CUR_DIR -iname "Cargo.toml")))

  cd $ROOT_DIR
  # build project
  cargo clean
  cargo build --release
  cd $CUR_DIR
}

openssl_encryption() {
  local ACTION="$1"
  local KEY="$2"
  local IN_FILE=$(find .. -iname "$3")

  if [ ! -z "$IN_FILE" ]; then
      local BIN_FOLDER=$(dirname $IN_FILE)
      local OUT_FILE="$BIN_FOLDER/$4"
  fi

  local OPENSSL_ENC=$(find .. -iname "${PROJECT}_enc")
  local OPENSSL_DEC=$(find .. -iname "${PROJECT}_dec")

  if [ ! -f "$OPENSSL_ENC" ] || [ ! -f "$OPENSSL_DEC" ]; then
      echo "Fatal! Binaries not found! Build project first!"
      exit 1
  fi

  case "$ACTION" in
      encode)
          local OPENSSL_BIN=$OPENSSL_ENC ;;
      decode)
          local OPENSSL_BIN=$OPENSSL_DEC ;;
      *)
          echo $"Error: Action not implemented!"
          exit 1
  esac

  if [ ! -z "$OPENSSL_BIN" ] && [ ! -z "$IN_FILE" ] && [ ! -z "$OUT_FILE" ] && [ ! -z "$KEY" ]; then
      if [ -f "$OUT_FILE" ]; then
          rm -f "$OUT_FILE"
      fi
      $OPENSSL_BIN $IN_FILE $OUT_FILE $KEY
  fi

  echo "$OUT_FILE"
}

parse_options() {
  case "$1" in
      openssl)
          PROJECT="openssl" ;;
      ring)
          PROJECT="ring" ;;
      *)
          echo "Use: test.sh <encryption name>"
          echo "Encryption name: openssl, ring"
          exit 1
  esac
  echo "Encription name: $PROJECT"
}

parse_options $@

KEY=$(generate_key)

if [ -z "$KEY" ]; then
    echo "Error: can't generate key :("
    exit 1
fi

build_target

RESULT_BIN=$(openssl_encryption "encode" "$KEY" "$INPUT_FILE" "$BIN_FILE")
RESULT_TXT=$(openssl_encryption "decode" "$KEY" "$BIN_FILE" "$DECRYPTED_FILE")

# Inject error
# echo "hahaha" >> "$RESULT_TXT"

DIFF_RESULT=$(diff $(find .. -iname "$INPUT_FILE") "$RESULT_TXT")

if [ ! -z "$DIFF_RESULT" ]; then
    echo "Error: file \"" $(find .. -iname "$INPUT_FILE") "\"<> \"" "$RESULT_TXT\""
    echo "$DIFF_RESULT"
    echo "Done: FAIL"
    exit 1
else
    echo "Done: SUCCESS"
fi
