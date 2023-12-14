use crate::Rule;
use indoc::formatdoc;
use pest::iterators::{Pair, Pairs};

pub fn parse(ast: Pairs<'_, Rule>) -> String {
	let mut out = String::new();

	for node in ast.clone() {
		out.push_str(&parse_node(node));
	}

	out
}

fn parse_node(node: Pair<'_, Rule>) -> String {
	let mut out = String::new();

	let children = node.clone().into_inner().collect::<Vec<Pair<'_, Rule>>>();

	match node.as_rule() {
		Rule::struc => {
			let fields = &children[1..];
			out.push_str(&formatdoc!(
				r#"
					create_{}:
					{}
				"#,
				children[0].as_str(),
				fields
					.iter()
					.map(|p| "\tlda 0\n\tsta $0; ".to_owned() + p.as_str())
					.collect::<Vec<String>>()
					.join("\n")
			));
		}
		_ => {
			for n in node.into_inner() {
				out.push_str(&parse_node(n));
			}
		}
	}

	out
}
