if ! command -v sqlx &>/dev/null; then
	echo "sqlx could not be found, use 'cargo install sqlx-cli'"
	exit 1
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
