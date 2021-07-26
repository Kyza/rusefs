extern crate clap;

use std::cmp;
use std::path::{Path, PathBuf};
use std::{env, fs};

use clap::clap_app;

use regex::Regex;

use walkdir::{DirEntry, WalkDir};

use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

fn main() {
	let matches = clap_app!(rusefs =>
		(version: "0.2.0")
		(author: "Kyza")
		(about: "Search your filesystem quickly using regex.")
		(@arg FOLDER: -f --folder +multiple +takes_value "The folder to search. Defaults to the current directory.")
		(@arg NAME: -n --name +multiple +takes_value "The regex to search for in the file name.")
		(@arg CONTENTS: -c --contents +multiple +takes_value "The regex to search for in the file contents.")
		(@arg MAX_SIZE: -s --max-size +takes_value "The maximum file size allowed to search in the file contents (default 100MB).")
		(@arg EXCLUDE: -e --exclude +multiple +takes_value "The regex to exclude searching files and folders.")
	)
	.get_matches();

	let config_file_path = Path::new(env::current_exe().unwrap().to_str().unwrap())
		.parent()
		.unwrap()
		.join("rusefs-config.toml");

	let mut settings = config::Config::default();
	settings
		.set_default("max_size", 100000000)
		.unwrap()
		.set_default("exclude", vec![] as Vec<String>)
		.unwrap()
		.merge(
			config::File::with_name(config_file_path.to_str().unwrap_or("rusefs-config.toml"))
				.required(false),
		)
		.unwrap();

	let mut max_size: u64 = settings.get_int("max_size").unwrap() as u64;
	let exclude_values = settings.get_array("exclude").unwrap();
	// Converts the `exclude_values` array into a `str` array.
	let mut exclude_strings = exclude_values
		.iter()
		.map(|exclude_value| exclude_value.to_string())
		.collect::<Vec<_>>();

	let mut folders: Vec<&str> = matches.values_of("FOLDER").unwrap_or_default().collect();
	if folders.len() == 0 {
		folders = vec!["."];
	}
	let names: Vec<&str> = matches.values_of("NAME").unwrap_or_default().collect();
	let contentses: Vec<&str> = matches.values_of("CONTENTS").unwrap_or_default().collect();

	if names.len() == 0 && contentses.len() == 0 {
		println!("Please specify something to search.");
		return;
	}

	let user_max_size: f64 = matches
		.value_of("MAX_SIZE")
		.unwrap_or("0")
		.parse::<f64>()
		.unwrap();
	let user_exclude_strings: Vec<&str> = matches.values_of("EXCLUDE").unwrap_or_default().collect();

	for exclude_string in user_exclude_strings {
		exclude_strings.push(exclude_string.to_string());
	}

	println!("Excluding: \"{}\"", exclude_strings.join("\", \""));

	let exclude_regex = exclude_strings
		.iter()
		.map(|exclude_string| Regex::new(&exclude_string).unwrap())
		.collect::<Vec<_>>();

	// Convert the vector of strings to a vector of regexes.
	let mut name_regexes: Vec<Regex> = vec![];
	for name in names {
		name_regexes.push(Regex::new(name).unwrap());
	}
	let mut contents_regexes: Vec<Regex> = vec![];
	for contents in contentses {
		contents_regexes.push(Regex::new(contents).unwrap());
	}

	if user_max_size > 0.0 {
		max_size = (user_max_size * 1000000.0) as u64;
		println!("Max Content Search Size: {}MB", user_max_size);
	} else {
		println!("Max Content Search Size: {}MB", max_size / 1000000);
	}

	for folder in folders {
		if let Err(_err) = search_folder(
			&folder,
			&name_regexes,
			&contents_regexes,
			&exclude_regex,
			&max_size,
		) {
			println!("Failed to search.");
			println!("{}", _err);
		}
	}
}

fn should_exclude(entry: &DirEntry, exclude: &[Regex]) -> bool {
	let file_name = &entry.file_name().to_string_lossy();
	for ex in exclude {
		if ex.is_match(file_name) {
			return false;
		}
	}
	true
}

fn search_file_name(names: &[Regex], file_name: &str) -> bool {
	for name in names {
		if name.is_match(&file_name) {
			return true;
		}
	}
	false
}

fn write_color(stdout: &mut termcolor::StandardStream, color: termcolor::Color, text: String) {
	let result = stdout.set_color(ColorSpec::new().set_fg(Some(color)));
	match result {
		Ok(res) => res,
		Err(error) => panic!("Failed to change color {:?}", error),
	}
	write!(stdout, "{}", &text);
}
fn writeln_color(stdout: &mut termcolor::StandardStream, color: termcolor::Color, text: String) {
	let result = stdout.set_color(ColorSpec::new().set_fg(Some(color)));
	match result {
		Ok(res) => res,
		Err(error) => panic!("Failed to change color {:?}", error),
	}
	writeln!(stdout, "{}", &text);
}

fn search_file_contents(contentses: &[Regex], max_size: &u64, file_path: &str) {
	let mut stdout = StandardStream::stdout(ColorChoice::Always);

	if let Ok(metadata) = fs::metadata(&file_path) {
		if metadata.len() < *max_size && metadata.file_type().is_file() {
			if let Ok(file_contents) = fs::read_to_string(&*file_path) {
				let newlines = &file_contents
					.char_indices()
					.filter_map(|(ix, c)| if c == '\n' { Some(ix) } else { None })
					.collect::<Vec<_>>();

				for contents in contentses {
					let mut first_line = true;
					for capture in contents.find_iter(&file_contents) {
						if first_line {
							writeln_color(&mut stdout, Color::Cyan, format!("\n{}", &file_path));
							first_line = false;
						}
						write_color(
							&mut stdout,
							Color::Blue,
							format!(
								"{} ",
								newlines
									.binary_search(&capture.start())
									.unwrap_or_else(|x| x) + 1,
							),
						);

						let start = &capture.start();
						let end = &capture.end();

						let before = &file_contents[..*start].lines().collect::<Vec<_>>()[0];
						let after = &file_contents[*end..].lines().collect::<Vec<_>>()[0];

						write_color(&mut stdout, Color::White, format!("{}", before));
						write_color(&mut stdout, Color::Green, format!("{}", capture.as_str()));
						writeln_color(&mut stdout, Color::White, format!("{}", after));
					}
				}
			}
		}
	}
}

fn search_folder(
	folder: &str,
	names: &[Regex],
	contentses: &[Regex],
	exclude: &[Regex],
	max_size: &u64,
) -> Result<(), walkdir::Error> {
	if let Ok(folder_path_buf) = fs::canonicalize(PathBuf::from(&folder)) {
		let folder_path = folder_path_buf
			.to_str()
			.unwrap()
			.strip_prefix("\\\\?\\")
			.unwrap();
		println!("Searching: \"{}\"", folder_path);
	}

	for entry in WalkDir::new(folder)
		.follow_links(true)
		.into_iter()
		// Run this here to not recurse into folders that match.
		.filter_entry(|entry| should_exclude(entry, exclude))
		.filter_map(|e| e.ok())
	{
		if !entry.file_type().is_file() {
			continue;
		}
		let file_name = entry.file_name().to_str().unwrap();
		if let Ok(file_path_buf) = fs::canonicalize(PathBuf::from(&entry.path().to_path_buf())) {
			let file_path = file_path_buf
				.to_str()
				.unwrap()
				.strip_prefix("\\\\?\\")
				.unwrap();

			let searching_names = names.len() > 0;
			let searching_content = contentses.len() > 0;

			if searching_names {
				if search_file_name(&names, &file_name) {
					if searching_content {
						search_file_contents(&contentses, &max_size, &file_path);
					} else {
						println!("- {}", file_path);
					}
				}
			} else if searching_content {
				search_file_contents(&contentses, &max_size, &file_path);
			}
		}
	}

	Ok(())
}
