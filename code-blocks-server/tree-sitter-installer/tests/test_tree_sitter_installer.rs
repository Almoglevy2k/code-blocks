use tree_sitter_installer::*;

#[test]
fn test_install_and_load_parser() {
    let target_dir = tempfile::tempdir()
        .expect("failed to get tempdir")
        .into_path()
        .join("test_install_lang")
        .join("test-parser");

    let installer = SupportedParser::Rust.get_installer();

    let mut parser = installer
        .install_parser(&target_dir)
        .expect("failed to install lang");

    let src = "fn main() {}";

    let tree = parser.parse(src, None);

    insta::assert_snapshot!(tree.unwrap().root_node().to_sexp(), @"(source_file (function_item name: (identifier) parameters: (parameters) body: (block)))");

    let mut parser = installer
        .load_parser(&target_dir)
        .expect("failed to load dynamic language");

    let tree = parser.parse(src, None);

    insta::assert_snapshot!(tree.unwrap().root_node().to_sexp(), @"(source_file (function_item name: (identifier) parameters: (parameters) body: (block)))");
}