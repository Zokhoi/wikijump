/*
 * check.rs
 *
 * wikijump-locales-validator - Validate Wikijump's Fluent localization files
 * Copyright (C) 2021 Wikijump Team
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 */

use std::collections::HashMap;
use std::path::Path;
use std::{fs, process};
use unic_langid::LanguageIdentifier;

/// The "primary" locale, to compare other locales against.
///
/// This is defined as one which is always complete, containing
/// every message key used by the application.
///
/// Thus, we can compare all other locales to it, ensuring they
/// are equal or subsets, raising errors on any new message keys,
/// as they are either typos or removed keys.
const PRIMARY_LOCALE: LanguageIdentifier = langid!("en");

pub fn run<P: AsRef<Path>>(directory: P) {
    let directory = directory.as_ref();
    let mut return_code = 0;

    macro_rules! fail {
        ($(arg:tt)*) => {{
            return_code = 1;
            eprint!("!! ");
            eprintln!($(arg)*);
        }};
    }

    let mut components = Vec::new();
    let mut locales = HashMap::new();
    print_real_path(directory);

    // Walk through all the component directories
    for entry in fs::read_dir(directory).expect("Unable to read localization directory") {
        let path = entry.path();
        if !path.is_dir() {
            fail!("Found non-directory in localizations: {}", path.display());
            continue;
        }

        // Walk through all the locales for a component
        print_real_path(path);
        for entry in fs::read_dir(path).expect("Unable to read component directory") {
            let path = entry.path();
            if !path.is_file() {
                fail!("Found non-file in component directory: {}", path.display());
                continue;
            }

            match path.extension() {
                Some(ext) => {
                    let ext = ext.to_str().expect("Path is not valid UTF-8");

                    if !ext.eq_ignore_ascii_case("ftl") {
                        fail!(
                            "Found file with non-Fluent file extension: {} ({})",
                            ext,
                            path.display(),
                        );
                    }
                }
                None => {
                    fail!("Found file with no extension: {}", path.display());
                    continue;
                }
            }

            let locale_name = path
                .file_stem()
                .expect("No base name in locale path")
                .to_str()
                .expect("Path is not valid UTF-8");

            let locale: LanguageIdentifier = match locale_name.parse() {
                Ok(locale) => locale,
                Err(error) => {
                    fail!("Directory name is not a valid locale: {}", locale_name);
                    fail!("Error: {}", error);
                    continue;
                }
            };

            todo!();
        }
    }

    process::exit(return_code);
}

fn print_real_path(path: &Path) {
    let real_path = path.canonicalize().expect("Unable to canonicalize path");
    println!("Reading through {}...", real_path.display());
}
