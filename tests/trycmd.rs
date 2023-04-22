#[test]
fn trycmd() {
	trycmd::TestCases::new()
		.insert_var("[EXE]", "")
		.unwrap()
		.register_bins(trycmd::cargo::compile_examples([]).unwrap())
		.case("README.md");
}