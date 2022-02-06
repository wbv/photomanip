#!/bin/sh

# run this in a terminal to instantly re-run the test suite anytime you change
# anything in this project. if you work in an adjacent window, you'll instantly
# see your results as soon as you hit "save"

# NOTE: requires inotifywait (try `apt install inotify-tools` on ubuntu/debian)

while :; do
	inotifywait -e close_write,moved_to,create -rq . 
	clear
	cargo test
	rustdoc src/main.rs
done
