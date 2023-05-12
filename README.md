# Github Action Play

To create a merge/tag workflow for release builds but only with tags and on main.

Extract tag from scratch/Cargo.toml on merge to main.

Verify tag does not exist. If it does not then use output of that verification to tag main with the version from the
Cargo.toml, and send trigger with information about whether or not this is a new release with a proper version:
    `^v[0-9]+\.[0-9]+\.[0-9]+$`

Then key off trigger to do release only actions when the tag is new.
