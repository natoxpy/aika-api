#!/bin/bash

if ! command -v sqlx &>/dev/null; then
	echo "sqlx could not be found, use 'cargo install sqlx-cli'"
	exit 1
fi

# Create root directory for files
mkdir -p ~/.aika

# Create database
if [[ ! -e ~/.aika/core.db ]]; then
	echo '+ created database'
	echo -n "  "
	touch ~/.aika/core.db
fi

export DATABASE_URL="sqlite:///home/$USER/.aika/core.db"

migration() {
	sqlx database create

	echo "+ Migration info"
	echo -n "  "
	sqlx migrate info

	echo "+ Running migration"

	echo -n "  "
	sqlx migrate run
	echo ""
}

# SQLX must be installed
migration
