#!/bin/bash

# based on eclipse-iceoryx/iceoryx ice_env.sh
#
# Copyright (c) 2022 by Apex.AI Inc. All rights reserved.
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.
#
# SPDX-License-Identifier: Apache-2.0

CONTAINER_NAME_PREFIX="elkodon_env_"
CONTAINER_MEMORY_SIZE="8g"
CONTAINER_SHM_MEMORY_SIZE="4g"
DEFAULT_OS_VERSION="ubuntu:22.04"
ELKODON_PATH=$(git rev-parse --show-toplevel)

COLOR_RESET='\033[0m'
COLOR_GREEN='\033[1;32m'
COLOR_CYAN='\033[1;34m'
FONT_BOLD='\033[1m'
COLOR_RED='\033[1;31m'

setup_docker_image() {
    echo "Europe/Berlin" > /etc/timezone
    ln -sf /usr/share/zoneinfo/Europe/Berlin /etc/localtime

    # ubuntu/debian and derivatives
    if command -v apt &>/dev/null; then
        apt update
        apt -y install sudo git fish curl vim lsb-release software-properties-common gcc libacl1-dev libclang-dev
    elif command -v pacman &>/dev/null; then
        pacman -Syu --noconfirm fish curl git vim clang
    else
        echo Please install the following packages to have a working elkodon environment:
        echo fish curl
    fi

    useradd testuser1
    useradd testuser2
    groupadd testgroup1
    groupadd testgroup2

    git config --global --add safe.directory /elkodon
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    export PATH=$PATH:/root/.cargo/bin
    rustup toolchain install stable
    rustup toolchain install nightly

    mkdir -p /root/.config/fish
    echo "set -gx PATH /root/.cargo/bin \$PATH" >> /root/.config/fish/config.fish
    exit
}

start_docker_session() {
    bash
    exit
}

help() {
    echo
    echo -e "${FONT_BOLD}elkodon development environment${COLOR_RESET}"
    echo
    echo -e "  $0 ${COLOR_CYAN}[ACTION] ${COLOR_RESET}(optional)${COLOR_CYAN}[DOCKER_OS]"
    echo
    echo -e "${COLOR_CYAN}ACTION:${COLOR_RESET}"
    echo -e "  ${FONT_BOLD}start${COLOR_RESET}          - start a specific docker container"
    echo -e "  ${FONT_BOLD}stop${COLOR_RESET}           - stop a specific docker container"
    echo -e "  ${FONT_BOLD}stop_all${COLOR_RESET}       - stops all docker containers running an elkodon environment"
    echo -e "  ${FONT_BOLD}enter${COLOR_RESET}          - enters (and starts if not running) the docker container"
    echo -e "  ${FONT_BOLD}remove${COLOR_RESET}         - removes a specific docker container"
    echo -e "  ${FONT_BOLD}remove_all${COLOR_RESET}     - removes all docker containers running an elkodon environment"
    echo -e "  ${FONT_BOLD}list${COLOR_RESET}           - list all docker containers running an elkodon environment"
    echo -e "  ${FONT_BOLD}list_running${COLOR_RESET}   - list all running docker containers running an elkodon environment"
    echo
    echo -e "${COLOR_CYAN}DOCKER_OS:${COLOR_RESET}"
    echo "  Defines the operating system of the docker container."
    echo "  Some standard options:"
    echo "    archlinux"
    echo "    ubuntu:22.04"
    echo "    ros:rolling"
    echo
    echo -e "${COLOR_CYAN}Example:${COLOR_RESET}"
    echo "  $0 start archlinux     # starts an elkodon docker container based on archlinux"
    echo "  $0 enter ubuntu:22.04  # enters (and starts if not running) an elkodon docker container based ubuntu"
    echo
    exit
}

create_docker() {
    echo -e "  ${COLOR_CYAN}create docker container${COLOR_RESET} [${FONT_BOLD}$CONTAINER_NAME${COLOR_RESET}]"
    docker run --name $CONTAINER_NAME \
               --mount type=bind,source=${ELKODON_PATH},target=/elkodon \
               --hostname ${OS_VERSION} \
               -dt --memory $CONTAINER_MEMORY_SIZE \
               --shm-size $CONTAINER_SHM_MEMORY_SIZE ${OS_VERSION}
    echo -e "  ${COLOR_CYAN}setting up elkodon development environment${COLOR_RESET} [${FONT_BOLD}$CONTAINER_NAME${COLOR_RESET}]"

    docker exec -it $CONTAINER_NAME /elkodon/$(realpath $0 --relative-to=$ELKODON_PATH) setup $OS_VERSION
}

startup_docker() {
    echo -en "         start elkodon development environment [${FONT_BOLD}$CONTAINER_NAME${COLOR_RESET}]"
    docker start $CONTAINER_NAME > /dev/null
    echo -e "\r  [${COLOR_GREEN}done${COLOR_RESET}]"
}

list_docker() {
    docker container ls -a | sed -n "s/.*\(elkodon_env_.*\)/  \1/p"
}

list_running_docker() {
    docker container ls | sed -n "s/.*\(elkodon_env_.*\)/  \1/p"
}

start_docker() {
    if [[ $(docker container inspect -f '{{.State.Running}}' $CONTAINER_NAME 2> /dev/null) == "true" ]]; then
        return
    fi

    if [[ $(list_docker | grep ${CONTAINER_NAME} | wc -l) == "0" ]]; then
        create_docker
    else
        startup_docker
    fi

    echo
    echo -e "  ${COLOR_CYAN}elkodon development environment${COLOR_RESET}"
    echo -e "  #################################################"
    echo
    echo -e "    container name..........: ${FONT_BOLD}${CONTAINER_NAME}${COLOR_RESET}"
    echo -e "    OS-Version..............: ${FONT_BOLD}${OS_VERSION}${COLOR_RESET}"
    echo -e "    memory..................: ${FONT_BOLD}${CONTAINER_MEMORY_SIZE}${COLOR_RESET}"
    echo -e "    shared memory...........: ${FONT_BOLD}${CONTAINER_SHM_MEMORY_SIZE}${COLOR_RESET}"
    echo -e "    elkodon-path............: ${FONT_BOLD}${ELKODON_PATH}${COLOR_RESET}"
    echo
}

stop_docker() {
    if [[ $(docker container inspect -f '{{.State.Running}}' $CONTAINER_NAME) == "true" ]]; then
        echo -en "         stopping elkodon development environment [${FONT_BOLD}${CONTAINER_NAME}${COLOR_RESET}] container"
        docker container stop $CONTAINER_NAME > /dev/null
        echo -e "\r  [${COLOR_GREEN}done${COLOR_RESET}]"
    fi
}

stop_all_docker() {
    echo -e "${COLOR_CYAN}stopping all elkodon development environment docker containers${COLOR_RESET}"
    for DOCKER in $(list_running_docker); do
        CONTAINER_NAME=$DOCKER
        stop_docker
    done
}

drop_docker() {
    stop_docker
    echo -en "         removing elkodon development environment [${FONT_BOLD}${CONTAINER_NAME}${COLOR_RESET}] container"
    docker rm $CONTAINER_NAME > /dev/null
    echo -e "\r  [${COLOR_GREEN}done${COLOR_RESET}]"
}

drop_all_docker() {
    echo -e "${COLOR_RED}removing all elkodon environment docker containers${COLOR_RESET}"
    for DOCKER in $(list_docker); do
        CONTAINER_NAME=$DOCKER
        drop_docker
    done
}

enter_docker() {
    start_docker

    docker exec -it $CONTAINER_NAME fish -c "
    echo
    eval 'echo \"  rustup version...........: \"\\033\[1\;37m(rustup --version | head -1 )\\033\[0m'
    eval 'echo \"  rust version.............: \"\\033\[1\;37m(rustc --version )\\033\[0m'
    echo
    cd /elkodon
    fish"

    # we use eval here since we would like to evaluate the expression inside of the docker
    # container and not right away in this script
    if [[ $? -ne 0 ]]; then
        docker exec -it $CONTAINER_NAME bash -c "
        echo
        eval 'echo \"  rustup version...........: \"\\033\[1\;37m(rustup --version | head -1 )\\033\[0m'
        eval 'echo \"  rust version.............: \"\\033\[1\;37m(rustc --version )\\033\[0m'
        echo
        cd /elkodon
        bash
        "
    fi
}

ACTION=$1
OS_VERSION=$2

if [[ -z $OS_VERSION ]]; then
    OS_VERSION=$DEFAULT_OS_VERSION
fi

CONTAINER_NAME=${CONTAINER_NAME_PREFIX}$(echo ${OS_VERSION} | tr : .)

if [[ $ACTION == "start" ]]; then
    start_docker
elif [[ $ACTION == "stop" ]]; then
    stop_docker
elif [[ $ACTION == "stop_all" ]]; then
    stop_all_docker
elif [[ $ACTION == "remove" ]]; then
    drop_docker
elif [[ $ACTION == "remove_all" ]]; then
    drop_all_docker
elif [[ $ACTION == "enter" ]]; then
    enter_docker
elif [[ $ACTION == "setup" ]]; then
    setup_docker_image
elif [[ $ACTION == "list" ]]; then
    list_docker
elif [[ $ACTION == "list_running" ]]; then
    list_running_docker
else
    help
fi
