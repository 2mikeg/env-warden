#!/bin/bash
set -euo pipefail

RESET="\033[0m"
BOLD="\033[1m"
RED="\033[31m"
GREEN="\033[32m"
YELLOW="\033[33m"
CYAN="\033[36m"
MAGENTA="\033[35m"

DB_DIR="$HOME/.local/share"
DB_NAME="env-warden.sqlite"
DB_PATH="$DB_DIR/$DB_NAME"
BIN_PATH="/opt/env-warden/env-warden"
SYMLINK_PATH="/usr/local/bin/env-warden"
INSTALL_DIR="/opt/env-warden"

error_exit() {
    printf "${BOLD}${RED}❌ ERROR:${RESET} %s\n" "$1" >&2
    exit 1
}

printf "${BOLD}${CYAN}🗑️ Uninstalling env-warden...${RESET}\n"

if [ -f "$SYMLINK_PATH" ]; then
    printf "${YELLOW}🔗 Removing symlink...${RESET}\n"
    sudo rm -f "$SYMLINK_PATH" || error_exit "Failed to remove symlink"
fi

if [ -f "$BIN_PATH" ]; then
    printf "${YELLOW}🗑️ Removing binary...${RESET}\n"
    sudo rm -f "$BIN_PATH" || error_exit "Failed to remove binary"
fi

if [ -d "$INSTALL_DIR" ]; then
    printf "${YELLOW}📂 Removing installation directory...${RESET}\n"
    sudo rm -rf "$INSTALL_DIR" || error_exit "Failed to remove installation directory"
fi

read -p "❓ Do you want to delete the database at $DB_PATH? (y/n) " -n 1 -r
printf "\n"
if [[ $REPLY =~ ^[Yy]$ ]]; then
    if [ -f "$DB_PATH" ]; then
        printf "${YELLOW}🗑️ Removing database...${RESET}\n"
        rm -f "$DB_PATH" || error_exit "Failed to remove database"
        printf "${GREEN}✅ Database removed successfully!${RESET}\n"
    else
        printf "${MAGENTA}⚠️ Database file not found. Skipping...${RESET}\n"
    fi
else
    printf "${MAGENTA}⚠️ Database not removed.${RESET}\n"
fi

printf "${GREEN}✅ Uninstallation complete!${RESET}\n"