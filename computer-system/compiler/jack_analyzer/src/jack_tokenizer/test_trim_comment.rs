#[cfg(test)]
mod tests {
	use super::*;

	mod trim_comment {
		use super::*;

		#[test]
		fn test_slash() {
			let query = "  var int i; // comment";
			let res = trim_comment(&query);
			assert_eq!(res, "  var int i; ")
		}
	}
}
