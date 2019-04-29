## Chit: Crate help in terminal

A tool for looking up details about rust crates without going to crates.io.

### Installation

Cargo installed already? `cargo install chit`.

If not... [find out how!](https://doc.rust-lang.org/cargo/getting-started/installation.html)


### Use

`chit rocket` OR `chit --crate=rocket`

<img src="https://github.com/peterheesterman/chit/blob/master/readme-images/chit.png?raw=true" width="496"  height="386"/>

`chit versions --crate=hex-buffer-serde`

<img src="https://github.com/peterheesterman/chit/blob/master/readme-images/versions.png?raw=true" width="391"  height="93"/>

`chit owners --crate=hex-buffer-serde`

<img src="https://github.com/peterheesterman/chit/blob/master/readme-images/owners.png?raw=true" width="390"  height="93"/>


### Chit should help answer these questions:
  - Who wrote this crate? What else did they write?
  - How old is this crate?
  - What versions are there? When did they come out?
  - What are the downloads over time?
  - Should i use this crate? 
  - How mature is it? <Star rating>

### Roadmap: 
(Would love Pull requests that build towards these objectives)
 - [ ] Tests
   - [x] Versions
   - [ ] Owners
   - [ ] Crate details
 - [ ] Add a CI pipeline that runs the tests
 - [ ] Change error handling to use result types
 - [ ] Use some more functional programming techniques to cut the code size down
 - [ ] Support windows by removing the use of color when windows binary is being built
 - [ ] distribute through brew, apt-get, etc...
