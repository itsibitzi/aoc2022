# aoc 2022

The code behind Alexandria Ocasio-Cortez's Christmas 2022 re-election campaign!

## Notes

- In order to make this fun I'm aiming for "decent" performance. This mainly means avoiding excessive amounts of allocation.
- I've generally used `fs::read_to_string` and then `.lines()` as a lazy way of parsing the input files. This is quite slow in Rust since it will validat the input bytes legal are UTF-8, etc. Performance could be imporved by using a crate like `bytelines`.
