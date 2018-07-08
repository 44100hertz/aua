mod lexer;

fn main() {
    let input: Vec<_> = std::fs::read_to_string("tests/lexer.txt").unwrap()
        .chars().collect();
    let items = lexer::lex(&input[..]);
    for item in items.unwrap().into_iter() {
        println!("{}", lexer::format_item(&input[..], item));
    }
}
