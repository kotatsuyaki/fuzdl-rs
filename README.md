Documentation generated by `rustdoc` for the master branch is available at <https://kotatsuyaki.github.io/fuzdl-rs/fuzdl>.


# Testing

The tests for `StateType::new` constructors has special requirements to be run.

- The tests depend on `chromedriver` being available in `$PATH`.
- Each test independently runs and stops its own instance of `chromedriver`.
  Since two instances of `chromedriver` cannot be running at the same time,
  the tests has to be run sequentially by `cargo test -- --test-threads 1`.
- The tests depend on the page layouts of Comic FUZ, and is designed to fail when the page layout changes.
