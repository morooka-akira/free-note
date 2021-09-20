use regex::Regex;

pub fn tokenize() {
	println!("start analyze")
}

pub fn trim_comment(code: &str) -> String {
	let trim = Regex::new(r"//.+").unwrap().replace_all(code, "");
	return trim.to_string();
}

#[cfg(test)]
mod tests {
	use super::*;

	mod trim_comment {
		use super::*;
		#[test]
		fn test_not_include() {
			let query = "  var int i;";
			let res = trim_comment(&query);
			assert_eq!(res, query)
		}

		#[test]
		fn test_slash() {
			let query = "  var int i; // comment";
			let res = trim_comment(&query);
			assert_eq!(res, "  var int i; ")
		}
	}
}
