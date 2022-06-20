#!/bin/sh

# run this in a terminal to instantly re-run the test suite anytime you change
# anything in this project. if you work in an adjacent window, you'll instantly
# see your results as soon as you hit "save"

# NOTE: requires inotifywait (try `apt install inotify-tools` on ubuntu/debian)

INOTIFYWAIT="$(command -pv inotifywait)"

if [ "$INOTIFYWAIT" = "" ]; then
	echo "E: command 'inotifywait' not found." >&2
	echo " (Note: try \`apt install inotify-tools\`)" >&2
	exit 127 # command not found
fi

while :; do
	"$INOTIFYWAIT" -e close_write,moved_to,create -rq .
	clear
	cargo test --verbose
	rustdoc src/main.rs
done
