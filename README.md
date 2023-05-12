# Github Action Play

To create a merge/tag workflow for release builds but only with tags and on main.

Extract tag from Cargo.toml on merge to main.

Verify tag does not exist.

Then use output of that verification to tag main with github action if tag is new.
