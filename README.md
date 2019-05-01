[![Build Status](https://dev.azure.com/chitbuilds/chit/_apis/build/status/peterheesterman.chit?branchName=master)](https://dev.azure.com/chitbuilds/chit/_build/latest?definitionId=1&branchName=master)
[![Downloads](https://img.shields.io/crates/d/chit.svg)](https://crates.io/crates/chit)
[![Stars](https://img.shields.io/github/stars/peterheesterman/chit.svg?style=popout
)](https://github.com/peterheesterman/chit/stargazers)
[![Collaborators](https://img.shields.io/github/contributors/peterheesterman/chit.svg)](https://github.com/peterheesterman/chit/graphs/contributors)
## Chit: Crate help in terminal

A tool for looking up details about rust crates without going to crates.io.


### Chit helps answer these questions:
  - Who wrote this crate? What else did they write?
  - What alternatives are there?
  - How old is this crate?
  - What versions are there? When did they come out?
  - What are the downloads over time?
  - Should i use this crate? 
  - How mature is it? <Star rating>
  

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


### Roadmap: 
(Would love Pull requests that build towards these objectives)
 - [x] Add badges
   - [x] Builds
   - [x] Downloads
   - [x] Stars
   - [x] Collaborators
 - [ ] Tests
   - [x] Versions
   - [ ] Owners
   - [ ] Crate details
 - [x] Add a CI pipeline that runs the tests
 - [ ] Change error handling to use result types
 - [ ] Use some more functional programming techniques to cut the code size down
 - [ ] Support windows by removing the use of color when windows binary is being built
 - [ ] distribute through brew, apt-get, etc...
