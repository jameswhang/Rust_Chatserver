# Rust Chatserver
A non-blocking chatserver written in Rust that implements Connect4 games.

## Contributors
Adel Lahlou, Diane Liu, James Whang, Nevil George

## Usage

* Start a server first: `cargo run s 127.0.0.1:8080`
* Start a client: `cargo run c 127.0.0.1:8080`
* Enter an ID name in the command line prompt
* Select a room to join
* Play!

## Features
* Uses mio Rust library to implement non-blocking server
* Game level message passing built atop chat server level message passing (using TCP sockets)
* Play Connect4 online
