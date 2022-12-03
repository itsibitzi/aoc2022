# aoc 2022

The code behind Alexandria Ocasio-Cortez's Christmas 2022 re-election campaign!

## Notes

- I've not included IO in time taken figures
- In order to make this fun I'm aiming for "decent" performance. This mainly means...
  - Avoiding excessive amounts of allocation.
  - Prefering `O(1)` indexing over `O(n)` iterations for lookups
- I've generally used `fs::read_to_string` and then `.lines()` as a lazy way of parsing the input files. This is quite slow in Rust since it will validate the input bytes legal are UTF-8, etc. Performance could theoretically be imporved by using a crate like `bytelines`, but after some very light testing it seems worse. Possibly the `bytelines` library isn't as well implemented?
