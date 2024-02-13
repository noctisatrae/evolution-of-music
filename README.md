# Understanding the evolution of music
## An attempt by noctis
The goal with this project was to take a shot at applying some theorical data science concept to a real life scenario.

## Outline of the project structure
In the `src/` directory you will find the source code of the data miner/analyser using Spotify's API. In order to use it, you will need to replace the secrets by your own. To compile it, you just need to run `cargo run` and you can use `cargo run -- --help` to see how to use it. Also take a look at the makefile for example, I can't assure it will work for you though!

The data will be saved by the parser in the `snapshot/` (you may need to create the `cleaned` and `uncleaned` directory for it to work). Along with all these precious information, you will be able to run the PCA analysis. Feel free to use your own with the uncleaned data.

## License 
The **code** of this project is proudly licensed under the GPLv3 standard. Have fun hackers!