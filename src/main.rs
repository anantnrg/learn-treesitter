use tree_sitter::{Parser, Point};
use tree_sitter_rust::language;

const HIGHLIGHTS: &'static str = include_str!("rust.scm");

fn main() {
    // Initialize source
    let code = r#"fn main() {
    println!("hello world");
}"#;
    let rope = ropey::Rope::from(code);

    // Initialize treesitter
    let mut parser = Parser::new();
    parser
        .set_language(language())
        .expect("Couldnt set the language");

    let tree = parser
        .parse_with(
            &mut |byte: usize, _position: Point| -> &[u8] {
                if byte < rope.len_bytes() {
                    rope.slice(byte..).as_str().unwrap().as_bytes()
                } else {
                    &[]
                }
            },
            None,
        )
        .unwrap();

    println!("{:?}", tree.root_node());
}
