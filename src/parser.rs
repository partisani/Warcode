use crate::Rule;
use indoc::formatdoc;
use pest::iterators::{Pair, Pairs};

pub fn parse(ast: Pairs<'_, Rule>) -> String {
	let mut out = String::new();

	for node in ast.clone() {
		let children = node.clone().into_inner().collect::<Vec<Pair<'_, Rule>>>();

		out.push_str(&match node.as_rule() {
			Rule::struc => gen_struc(children),
			Rule::fndec => gen_fndec(children),
			_ => String::new(),
		});
	}

	out
}

fn gen_struc(children: Vec<Pair<'_, Rule>>) -> String {
	return formatdoc! {
		r#"
			{consts}
			create_{name}:
				plx
				lda #0
			{code}
		"#,
		name = children[0].as_str(),
		code = {
			let fields = children[1..].iter().map(|f| f.as_str());
			fields.map(|f| "\tsta $0, x ; zero-paged address ; field ".to_owned() + f + "\n\tinx").collect::<Vec<String>>().join("\n")
		},
		consts = {
			let fields = children[1..].iter().map(|f| f.as_str()).enumerate();
			fields.map(|f| "player.".to_owned() + f.1 + " = " + &f.0.to_string()).collect::<Vec<String>>().join("\n")
		}
	};
}

fn gen_fndec(children: Vec<Pair<'_, Rule>>) -> String {
	return formatdoc! {
		r#"
			{label}:
			{code}
		"#,
		label = children[0].as_str(),
		code = {
			"\trts"
		}
	};
}
