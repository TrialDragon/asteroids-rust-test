# asteroids-rust-test

A re-make / clone of the famous 1979 game Asteroids in the Bevy engine.

## Screenshots

![title screen](https://imgur.com/kbiPXSb)
![play area](https://imgur.com/SfTQSzy)
![game over](https://imgur.com/VWI7nst)

## Running

To run the game with dev tools, just do:
`cargo run`

To run without dev tools, do:
`cargo run --no-default-features`
> This disables the default `dev` feature which enables the dev tools.

To run the release, do:
`cargo run --no-default-features --release`

To build the release, do:
`cargo build --no-default-features --release`
> [!NOTE]
> For building, you must also have the assets in an `assets` folder
> of the same directory as the executable.
> Easiest to just do something like:
> `cp -r assets/ <executables_directory>/assets`
