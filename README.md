Infos:

- Raspberrypi touchscreen: 800x400 pixels

# Build

## Build with nix

```bash
# this is an alias to scripts/prepare-sqlx
prepare-sqlx

nix build

```

# Rust Project structure

- backend (server functions)
  - database
    > things only related to database interactions
    - models
      > structs, new, trait implementation for general purpose traits (Display, Debug)
    - behaviours
      > a interaction with the database that can be performed i.e.: create_user.rs send_money.rs
    - misc
      > things that don´t fit into either models or behaviours
  - convert
    > conversion between database types and backend types \
    > prefer implementing the [From](https://doc.rust-lang.org/std/convert/trait.From.html) \
    > file name: `{StructNameA}_from_{StructNameB}.rs`
  - core
    > code that does not interact with the database directly
    - models
      > structs, new, trait implementation for general purpose traits ( Display, Debug)
    - behaviours
      > a interaction with a backend \
      > may include multiple interactions with the database
    - misc
      > things that don´t fit into either models or behaviours
  - shared
    > helper functions that are shared in the entire backend)
- convert
  > conversion between frontend types and backend types \
  > prefer implementing the [From](https://doc.rust-lang.org/std/convert/trait.From.html) \
  > file name: `{StructNameA}_from_{StructNameB}.rs`
- frontend
  > code run on the client / at hydration
  - component
    > reusable components (most things go here)
    - icons
      > all used icons should be extracted into a own component
    - user
      > everything related to users, if in doubt try avoiding this folder in favour of the following ones
    - transaction
      > everything related to transactions
    - article
      > everything related to articles
  - routes
    > page views composed of components
    > the files in this directory should be tiny
    > if they get to complex create a component instead
  - shared
    > helper functions that are shared in the entire frontend
- shared
  > helper functions that are needed in the backend and in the frontend
