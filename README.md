# Static Media Server

Serves static media files in various formats

## General Information

Pick a directory on your computer, put media files in it, run this program and those files will be available on the
local network.

## Setup

No external dependencies, just run `cargo build --release` and put the binary someplace in your `$PATH`.

## Usage

Run `static_media_server $IP:$PORT $DIR` with the arguments replaced with the actual values you want to use. You can
leave out the `$DIR` to use the current working directory.

## Project Status

It's early days, nothing really works yet.
