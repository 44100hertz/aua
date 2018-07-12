mod lexer;
mod asm6502;

fn main() {
    let input: Vec<_> = std::fs::read_to_string("tests/lexer.txt").unwrap()
        .chars().collect();
    let items = lexer::lex(&input[..]).unwrap();
    for item in items.iter() {
        println!("{}", lexer::format_item(&input[..], item.clone()));
    }
    asm6502::assemble(&input[..], &items[..]);
}
