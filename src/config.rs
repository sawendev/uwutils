// Easy configuration loader

use std::{collections::HashMap, path::Path};

pub fn get_cfg(path: impl AsRef<Path>) -> HashMap<String, String> {
	std::fs::read_to_string(path)
		.map(|config| {
			config
				.lines()
				.filter_map(|l| match l.split_once('=') {
					Some((l, r)) => Some((l.to_owned(), r.to_owned())),
					None => None,
				})
				.collect::<HashMap<_, _>>()
		})
		.unwrap_or(HashMap::new())
}
