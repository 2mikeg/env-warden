#!/bin/bash
set -euo pipefail  

RESET="\033[0m"
BOLD="\033[1m"
DIM="\033[2m"

RED="\033[31m"
GREEN="\033[32m"
YELLOW="\033[33m"
BLUE="\033[34m"
CYAN="\033[36m"
MAGENTA="\033[35m"

DB_DIR="$HOME/.local/share"
DB_NAME="my_database.sqlite"
DB_PATH="$DB_DIR/$DB_NAME"

error_exit() {
    printf "${BOLD}${RED}❌ ERROR:${RESET} %s\n" "$1" >&2
    exit 1
}

command -v sqlite3 >/dev/null 2>&1 || error_exit "sqlite3 is not installed. Please install it first."

printf "${BOLD}${CYAN}🔧 Setting up SQLite database...${RESET}\n"

printf "${BLUE}📂 Checking database directory...${RESET} "
mkdir -p "$DB_DIR" || error_exit "Failed to create directory $DB_DIR"
printf "${GREEN}OK ✅${RESET}\n"

if [ ! -f "$DB_PATH" ]; then
    printf "${YELLOW}🆕 Creating new database at:${RESET} ${MAGENTA}%s${RESET}\n" "$DB_PATH"
    sqlite3 "$DB_PATH" "PRAGMA journal_mode = WAL;" || error_exit "Failed to create database"
    printf "${GREEN}✅ Database created successfully!${RESET}\n"
else
    printf "${YELLOW}⚠️ Database already exists at:${RESET} ${MAGENTA}%s${RESET}\n" "$DB_PATH"
fi

printf "${BLUE}🔑 Setting write permissions...${RESET} "
chmod 664 "$DB_PATH" || error_exit "Failed to set permissions"
printf "${GREEN}Done ✅${RESET}\n"

printf "\n${BOLD}${CYAN}📄 Database Details:${RESET}\n"
ls -lh "$DB_PATH"

printf "\n${BOLD}${GREEN}🚀 Setup complete!${RESET}\n"
printf "📍 ${BOLD}Database path:${RESET} ${MAGENTA}%s${RESET}\n" "$DB_PATH"
