# Automove
Moves files matching a config into certain subdirectories.
Create a move.toml file in the directory and execute it from there.

## Example for move.toml
```
[[moves]]
pattern = "Big Buck"
path = "/home/user/OpenSourceMovies"

[[moves]]
pattern = "Elephants Dream"
path = "/home/user/OpenSourceMovies"
```