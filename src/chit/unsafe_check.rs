extern crate flate2;
extern crate geiger;
extern crate tar;

use super::sources;
use std::io::Read;

// TODO: Implement actual error type

// TODO: Implement a way to check if the entire crate forbids unsafe (i.e. #![forbid(unsafe_code)] in lib.rs / main.rs)
// Caveat: lib.rs might not be the root of the crate, as in Cargo.toml, you can set the root to be whatever you want.

// TODO: Maybe implement downloading dependencies and checking them for unsafe

/// Checks if a crate contains usages of unsafe
pub fn contains_unsafe(crate_name: &str, version: &str) -> Result<bool, ()> {
    // Downloading completely to memory should be fine, as almost every crate is under 1mb.
    // The biggest crate I'm aware of is `ring`, which is only 5.4 mb
    let data =
        sources::download_to_memory(sources::crates::download_url(crate_name, version)).ok_or(())?;

    let tar = flate2::read::GzDecoder::new(&data[..]);
    let mut archive = tar::Archive::new(tar);

    let is_safe = archive
        .entries()
        .unwrap()
        .filter_map(|c| {
            let mut entry = c.expect("Failed to unpack crate");

            if entry
                .path()
                //TODO: If this is a problem now, it will probably be a problem when the user downloads the crate (could gracefully fail and display a warning or something)
                .expect("Failed to parse path, non-utf8 path on windows?")
                .extension()?
                == "rs"
            {
                let mut src = String::new();
                entry
                    .read_to_string(&mut src)
                    .expect("Failed to read .rs file in .tar");
                Some(
                    geiger::find_unsafe_in_string(&src, geiger::IncludeTests::No)
                        .expect("Failed to parse rust file"),
                )
            } else {
                None
            }
        })
        .all(|c| c.forbids_unsafe || !c.counters.has_unsafe());

    Ok(!is_safe)
}
