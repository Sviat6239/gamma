pub mod frontend;
use frontend::lexer::lexer;

fn main() {
    let input = r#"declare greeting: [12] = "Hello World!";
                        print(greeting);
                        println;"#;

    let tokens = lexer(input);
    for token in tokens {
        println!("{:?}", token);
    }
}
