Contributing to Chit: Crate help in terminal
==

Thanks for taking the time to contribute to Chit! 🎉 Chit welcomes pull requests from everyone and anyone.

## Pull request process
 1. Fork the Chit project
 2. Create a new branch. The branch name should be prefixed with the authors github name, followed by one of [`chore`, `docs`, `feat`, `bugfix`, `refactor`, `test`]
 then the task. For example bobsmith/feat/add-logo 
 3. Make changes. Once the feature, bug or chore etc. is complete the commit message should contain whether it is a feature, bug or chore etc. for example "feat(Header): Add logo to header component"
 4. Make a pull request into master. For this you will need to push to your fork and [submit a pull request][pr].

[pr]: https://github.com/peterheesterman/chit/pulls


## Passing the build
 1. Run the auto formatter by using `cargo fmt` in terminal.
 2. Run the tests by using the `cargo test` in terminal.
 

## Alternatives

 Please add them, i will be very happily accept suggestions and PRs for more alternatives.

 Please note that currently chit pulls the `alternatives.json` from `peterheesterman/github` master directly so if you want to test adding more you will need to configure the alternatives module to read a local file for development purposes. 
