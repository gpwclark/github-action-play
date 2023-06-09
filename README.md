# Github Action Play

To create a merge/tag workflow for release builds but only with tags and on main.

Extract tags from scratch/Cargo.toml printer/Cargo.toml on merge to main.

Verify tag from toml file(s) does not exist and version of binary and lib matches. If it does not then use output of that verification to tag main with the version from the
Cargo.toml, and send trigger with information about whether or not this is a new release with a proper version:
    `^v[0-9]+\.[0-9]+\.[0-9]+$`

    i.e. a version starting with `v` followed by three numbers separated by `.` and no trailing characters

Then key off trigger to do release only actions when the tag is new.

# GitHub Pages

- Houses static sites on gh pages site. The `docs` directory houses the main index.html file. The 
`.github/workflows/gh-pages.yml` file builds the directories that `docs` references. New directories of static sites
can be added. Currently grcov and the crates docs are built.

## cargo docs

### Generating site
```
cargo doc --open
```
if needed in a specific folder use:
```
cargo doc --target-dir ./some-dir/
```

## cargo docs
cargo doc --target-dir docs/scratch

## grcov

### Installation
```
cargo install grcov
rustup component add llvm-tools-preview
```

### Generating report
```
cargo clean
export RUSTFLAGS="-Cinstrument-coverage"
cargo build
export LLVM_PROFILE_FILE="abc-%p-%m.profraw"
cargo test
grcov . -s . --binary-path ./target/debug/ -t html --branch --ignore-not-existing -o ./target/debug/coverage/
```

#### GH pages setup
- first time use, set up orphan branch.
```
BRANCH_NAME=gh-pages # Change to a desired branch name
git checkout --orphan $BRANCH_NAME
git rm -rf .
echo "Github pages branch" > README.md
git add README.md
git commit -m "Add readme"
git push origin $BRANCH_NAME
```

