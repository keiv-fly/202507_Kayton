mod vm;
mod write;
mod lexer;
mod parser;
mod codegen;
fn main() {
    write::println_to_console(b"Hello, World!");
}
