# Github Action Play

To create a merge/tag workflow for release builds but only with tags and on main.

Extract tag from Cargo.toml on merge to main.

Verify tag does not exist if it does not then use output of tkat verification to tag main with github action, and
send trigger if tag is new

Then key off trigger sent by build action to do release only actions when the tag is new.
