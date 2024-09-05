use rsshader::*;

fn main() {
    let mut source = "

        fn test(a: f32, b: f32) -> f32 {
            a + b
        }

    ".to_string();

    let mut errs = Vec::new();
    let stream = tokenize(&source, &mut errs);

    errs.sort();
    errs.reverse();
    for err in &errs {
        source.insert_str(err.span.start, "\x1b[31m");
        source.insert_str(err.span.end + "\x1b[31m".len(), "\x1b[0m");

        let end_of_line = source.char_indices().find(|(i, c)| *i > err.span.start && *c == '\n').map(|(i, _)| i).unwrap_or(source.len());

        source.insert_str(end_of_line, &format!("\x1b[31m   // {err}\x1b[0m"))
    }

    println!();
    println!("{source}");
    println!();
    println!("-----------------------------------------");
    println!();
    println!("{stream}");
    println!();
    println!("-----------------------------------------");

    for err in errs {
        println!();
        println!("{err}");
    }
}