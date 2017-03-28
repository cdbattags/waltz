mod utils;
use utils::*;

#[test]
fn simple() {
    given(
        r#"
        # Getting started

        First of all, create a simple `Cargo.toml` file:

        ```toml,file=Cargo.toml
        [package]
        authors = ["Pascal Hertleif <killercup@gmail.com>"]
        name = "foo"
        version = "0.1.0"
        ```

        Nice. Now you can put this into `src/main.rs`:

        ```rust,no_run,file=src/main.rs
        fn main() {
            println!("Hello, world!");
        }
        ```
    "#,
    )
            .running(waltz)
            .creates(
                file("Cargo.toml").containing(
                    r#"
        [package]
        authors = ["Pascal Hertleif <killercup@gmail.com>"]
        name = "foo"
        version = "0.1.0"
    "#,
                ),
            )
            .creates(
                file("src/main.rs").containing(
                    r#"
        fn main() {
            println!("Hello, world!");
        }
    "#,
                ),
            )
            .running(|cwd| main(cwd).prints("Hello, world!"));
}

#[test]
fn complex_paths() {
    given(
        r#"
        First off:

        ```toml,file=Cargo.toml
        [package]
        authors = ["Pascal Hertleif <killercup@gmail.com>"]
        name = "foo"
        version = "0.1.0"

        [[bin]]
        name = "lolwut"
        path = "src/bin/lolwut/main.rs"
        ```

        And then:

        ```rust,no_run,file=src/bin/lolwut/main.rs
        fn main() {
            println!("Sup dawg I herd u likd nested dirs");
        }
        ```
    "#,
    )
            .running(waltz)
            .creates(file("Cargo.toml"))
            .creates(file("src/bin/lolwut/main.rs"))
            .running(|cwd| binary(cwd, "lolwut").prints("Sup dawg"));
}
