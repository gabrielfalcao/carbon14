#!/usr/bin/env bash
##
## ##""""""""`# d8888b. ##""""""""`#    #""""""""#                     dP
## ##  mmmmmmm#     `88 ##  mmmmmmm#    #mmm  mmm#                     88
## #`      #### .aaadP' #`      ####    ####  #### .d8888b. .d8888b. d8888P
## ##  ######## 88'     ##  ########    ####  #### 88ooood8 Y8ooooo.   88
## ##  ######## 88.     ##  ########    ####  #### 88.  ...       88   88
## ##        .# Y88888P ##        .#    ####  #### `88888P' `88888P'   dP
## ############         ############    ##########
##

function report_mismatch() {
    x="$1"
    shift
    y="$1"
    1>&2 echo -e "\033[1;38;5;160mMismatch\033[0m"
    1>&2 echo -e "\033[1;38;5;220mi.e.:\033[0m"
    1>&2 echo -e "\033[1;38;5;196m${x} != ${y}\033[0m"
    exit 1
}
function report_ok() {
    1>&2 echo -e " \033[1;38;5;112mOK\033[0m i.e.: \033[1;38;5;220m $1\033[0m"
}

set -e

target="rust-toolchain.toml"

target_c14_path="rust-toolchain.toml.c14"

cargo build

if [ ! -e "${target_c14_path}" ]; then
    ./target/debug/carbon14 -of "${target_c14_path}" "${target}"
fi


target_sha256sum=$(sha256sum "${target}" | awk '{ print $1 }')
target_sha256sum_c14="$(grep sha256: "${target_c14_path}" | lastcol)"
target_sha384sum=$(sha384sum "${target}" | awk '{ print $1 }')
target_sha384sum_c14="$(grep sha384: "${target_c14_path}" | lastcol)"
target_sha512sum=$(sha512sum "${target}" | awk '{ print $1 }')
target_sha512sum_c14="$(grep sha512: "${target_c14_path}" | lastcol)"

echo -n "checking sha256sum of ${target} ..."
if [ "${target_sha256sum_c14}" != "${target_sha256sum}" ]; then
    report_mismatch "${target_sha256sum_c14}" "${target_sha256sum}"
else
    report_ok "${target_sha256sum_c14}"
fi

echo -n "checking sha384sum of ${target} ..."
if [ "${target_sha384sum_c14}" != "${target_sha384sum}" ]; then
    report_mismatch "${target_sha384sum_c14}" "${target_sha384sum}"
else
    report_ok "${target_sha384sum_c14}"
fi

echo -n "checking sha512sum of ${target} ..."
if [ "${target_sha512sum_c14}" != "${target_sha512sum}" ]; then
    report_mismatch "${target_sha512sum_c14}" "${target_sha512sum}"
else
    report_ok "${target_sha512sum_c14}"
fi
