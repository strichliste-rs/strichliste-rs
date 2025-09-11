# General information

This is a digital tally sheet for use in trusted environments.

You can test it out on https://demo.strichliste.rs

# Build

## Build with nix

```bash
nix build

```

## Build with cargo-leptos

```bash

SQLX_OFFLINE=true cargo leptos build --release
```

# Development

## With nix

If you have nix direnv

```bash
direnv allow .
```

Otherwise just do

```
nix flake develop
```

## Other instructions

The data directory flag (-d) is the directory the sqlite database will be placed.

```bash
# could also be present in a .env
export DATABASE_URL="sqlite:tmp/db.sqlite" # not needed with nix (env is in flake.nix)

sqlx database setup
cargo leptos watch -- -d ./tmp -c ./config_example.yaml
```

## Preparing for build

We use sqlx and compile-time checked queries. This has the drawback of needing a live db to check the queries, which is not possible in build environments. So we "cache" the sqlx queries using the following command:

```
# this is an alias to scripts/prepare-sqlx
prepare-sqlx
```

This step is needed in order for the build instructions to work. This step should be done after changing queries and before merging into main.
