# Rust Chatserver
A non-blocking chatserver written in Rust that implements Connect4 games.

## Contributors
Adel Lahlou, Diane Liu, James Whang, Nevil George

## Usage

* Start a server first: `cargo run s 127.0.0.1:8080`
* Start a client: `cargo run c 127.0.0.1:8080`
* Enter an ID name in the command line prompt to claim it
* Select a room to join by typing into its name
* Play is currently supported by directly typing in the command strings
* "join" allows you to join a connect four game if there is space
* "leave" lets you leave the game if you are in it
* Simply type your number to make a move
* After the game is over, the board becomes a playground until at least one of the players leave! 
* Because you are interacting directly with the message passing system, the user has to keep track of the state of the board. The program is aware of who is the winner and keeps track of the winner even after playing around with it, but there is no feedback to the user at this time.

## Features
* Uses mio Rust library to implement non-blocking server
* Game level message passing built atop chat server level message passing (using TCP sockets)
* Play Connect4 online
