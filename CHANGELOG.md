
# Change Log

### 0.1.15 - hyphenation
 - mthh fix a typo and added hyphens to the end of wrapping line when required.
 
### 0.1.14 - Make it bright
 - Change the color to be bright blue for text so that it is easier to read

### 0.1.13 - The nix version
 - Add repository url to repo command output
 - Change change get alternatives test to test syntax only 
 - Add repository url to repo command output
 - Change change get alternatives test to test syntax only 

### 0.1.12 - Pushing code quality
 - Change the 3 star threshold down to 750 from 1000 recent downloads
 - Add a new sub command repo - for things from github
 - Add a Owners type to the extraction model
 - Refactor owners to be a more functional flavour
 
### 0.1.11 - Refactor all the things
 - Refactor format module and add some tests
 - Refactor version to be a more functional flavour
 - Add a roadmap section to the readme
 - Added more logging options
 - Add error types and results for alternatives

### 0.1.10 - read the alternatives from github
 - luteberget reported and issue where a panic happens when listing alternatives.
 - 8176135 fix this! Champion!

## 0.1.9 - Let the people speak for real
 - Alternatives are set based because Incola suggested it and i like it a lot more
 
## 0.1.8 - Let the people speak - not released
 - Alternatives for some common crates
 - Show up in the details section of the chit
 
## 0.1.7 - Keep it simple . . . for now
 - Add crate categories to the output
 - Add crate description to the output
 
## 0.1.6 - The key is in the words
 - Refactor 
   - All the actions to their own file
   - The remove_quote method usage
 - Add keywords to the output

## 0.1.5 - A license to ... show
 - Include the software license for the shown crate

## 0.1.4 - A link to the ... docs
 - Add external resource links for docs, crates.io and repo (Thanks to lnicola for the suggestion!)
 - Fix border alignment is not an issue

## 0.1.3 - nanocryk & joseluis you rule!
 - Multi owner support - Shoutout to nanocryk!
 - Speed gains and making -V arg work properly - Shoutout to joseluis! 

## 0.1.2 - Bigger is better?
 - Include the size of the current version
 - Include the size of previous versions

## 0.1.1 - Great `crate help in terminal`
 - Make it clear what chit stands for in the docs
 - Add bug and feature issue templates

## 0.1.0 - Clapping my self silly
 - Use the command line argument parser (clap) crate - move the `owners` and `versions` code to there own subcommands
 - Introduce a CODE_OF_CONDUCT.md
 - Introduce a CONTRIBUTING.md
 - Add a pull request template
 - Add LICENSE.md

## 0.0.5 - All the versions + Star ratings
 - Print all the versions of a crate
 - Print the recent downloads of a crate
 - Print the star rating based on recent downloads

## 0.0.4 - Better error handling
 - Better error handling, less panic!
 - Better date format
 - Remove all the quotes in the output
 - Update the readme 

## 0.0.3 - Add other crates of owner
 - Change the order thing apear in the chit
 - Add details of the other crates of an owner
 - better padded write abstraction
 - update readme with an image

## 0.0.2 - Getting better
 - A change log
 - Better visuals 
 - Better code structure

## 0.0.1 - Initial release
  - First owner
  - Download count
  - Current version
