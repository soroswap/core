#!/bin/bash
# Declare colors
YELLOW="\033[43;30m"
GREEN="\033[42;30m"
RED="\033[41;30m"
BLUE="\033[44;30m"
PURPLE="\033[48;5;54m"
RESET="\033[0m"

# Function to display colored text
display_colored_text() {
    local color="$1"
    local text="$2"
    echo -e "\033[1m${!color}${text}${RESET}"
}