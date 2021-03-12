use std::{fs, path::PathBuf};
use anyhow::{anyhow, Context, Result};

pub fn check_daemon_path(path: PathBuf) -> Result<String> {
    let mut file_name = "";
    if let Some(f) = path.file_name().unwrap().to_str() {
	file_name = f;
    }

    if let Some(dir_path) = path.parent() {
	if let Some(path) = dir_path.to_str() {
	    let paths = fs::read_dir(path)
		.with_context(|| {
		    format!("failed to read {}", path)
		})?;
	    for path in paths {
		if let Some(fname) = path.unwrap().path().file_name() {
		    if fname == file_name {
			if let Ok(name) = fname.to_os_string().into_string() {
			    return Ok(name);
			}
		    }
		}
	    }
	}
    }
    Err(anyhow!("cannot file the daemon file"))
}

pub mod parser {
    use std::collections::HashMap;
    use std::fs;
    use pest::Parser;
    use anyhow::{anyhow, Context, Result};

    #[derive(Parser)]
    #[grammar = "pest/systemd.pest"]
    pub struct SystemdParser;

	#[derive(Debug, Clone)]
	pub enum SystemdValue {
		List(Vec<String>),
		Str(String),
	}

	fn preprocess_map(map: &mut HashMap<String, HashMap<String, SystemdValue>>) {
		for (_, value) in map.into_iter() {
			for (_, v) in value.into_iter() {
				if let SystemdValue::List(vs) = v {
					if vs.len() == 0 {
						*v = SystemdValue::Str("".to_string());
					} else if vs.len() == 1 {
						*v = SystemdValue::Str(vs[0].clone());
					}
				}
			}
		}
		// println!("{:#?}", map);
	}

    pub fn parse(filepath: &str) -> Result<HashMap<String, HashMap<String, SystemdValue>>>  {
		let unparsed_file = fs::read_to_string(filepath)
			.with_context(|| format!("cannot read file {}", filepath))?;

		let file = SystemdParser::parse(Rule::file, &unparsed_file)
			.expect("unsuccessful parse")
			.next()
			.unwrap();

		let mut properties: HashMap<String, HashMap<String, SystemdValue>> =
			HashMap::new();

		let mut current_section_name = "".to_string();
		let mut current_key_name = "".to_string();

		for line in file.into_inner() {
			match line.as_rule() {
			Rule::section => {
				let mut inner_rules = line.into_inner();
				current_section_name = inner_rules.next().unwrap().to_string();
			},
			Rule::property => {
				let mut inner_rules = line.into_inner();
				let section = properties.entry(current_section_name.clone()).or_default();
				let name: String = inner_rules.next().unwrap().to_string();
				let value: String = inner_rules.next().unwrap().to_string();

				if name == current_key_name {
					let entry = section.entry(current_key_name.clone()).or_insert(SystemdValue::List(vec![]));
					if let SystemdValue::List(ent) = entry {
						ent.push(value);
					}
				} else {
					let entry = section.entry(name.clone()).or_insert(SystemdValue::List(vec![]));
					if let SystemdValue::List(ent) = entry {
						ent.push(value);
					}
					current_key_name = name;
				}
			},
			Rule::EOI => (),
			_ => unreachable!(),
			}
		}
		preprocess_map(&mut properties);
		// println!("{:#?}", properties);
		Ok(properties)
	}
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
	use parser::SystemdValue;
	use std::env::current_dir;

    #[test]
    fn test_parse() {
		let mut dir = current_dir().unwrap();
		dir.push("src");
		dir.push("service");
		dir.push("cardano-node.service");

		let file_name = dir.to_str().unwrap();
		let prop = parser::parse(file_name).unwrap();

		println!("{:#?}", prop);


		// if let Some(payload) = prop.get(&"Unit".to_string()) {
		// 	let desc = payload.get(&"Description".to_string()).unwrap();
		// 	let after = payload.get(&"After".to_string()).unwrap();
		// 	let wants = payload.get(&"Wants".to_string()).unwrap();
		// 	if let (SystemdValue::Str(d), SystemdValue::Str(a), SystemdValue::Str(w)) = 
		// 		(desc, after, wants) {
		// 		assert_eq!(*d, "Cardano Node".to_string());
		// 		assert_eq!(*a, "network-online.target".to_string());
		// 		assert_eq!(*w, "network-online.target".to_string());
		// 	} else {
		// 		assert!(false);
		// 	}
		// } else {
		// 	assert!(false);
		// }

		// if let Some(payload) = prop.get(&"Service".to_string()) {
		// 	let typ = payload.get(&"Type".to_string()).unwrap();
		// 	let wanted_by = payload.get(&"WantedBy".to_string()).unwrap();
		// 	if let (SystemdValue::Str(t), SystemdValue::Str(w)) = (typ, wanted_by) {
		// 		assert_eq!(*t, "simple".to_string());
		// 		assert_eq!(*w, "multi-user.target".to_string());
		// 	} else {
		// 		assert!(false);
		// 	}
		// } else {
		// 	assert!(false);
		// }
	}
}
