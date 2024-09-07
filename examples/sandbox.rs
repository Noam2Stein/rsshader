use rsshader::*;
use tokenization::TokenStream;

fn main() {
    let mut source = "

        fn test(a: f32, b: f32) -> f32 {
            a + b + 5u31
        }

    ".to_string();

    let mut errs = Vec::new();
    let stream = TokenStream::parse(&source, &mut errs);

    source = {
        let mut lines = source.split("\n").collect::<Box<[&str]>>();
        let space_offset = lines.iter().map(|line| line.chars().position(|c| c != ' ').unwrap_or(usize::MAX)).min().unwrap();
        for line in &mut lines {
            *line = &line[space_offset.min(line.len())..line.len()];
        }

        let mut output = String::with_capacity(source.len() + lines.len() * (2 + lines.len().ilog10() as usize));

        let mut line_index = 1;
        for line in lines {
            output.push_str(&format!("{line_index: >3}\t{line}\n"));
            line_index += 1;
        }

        output
    };

    errs.sort();
    errs.reverse();
    for err in &errs {
        source.insert_str(err.span.start, "\x1b[31m");
        source.insert_str(err.span.end + "\x1b[31m".len(), "\x1b[0m");

        let end_of_line = source.char_indices().find(|(i, c)| *i > err.span.start && *c == '\n').map(|(i, _)| i).unwrap_or(source.len());
        
        source.insert_str(end_of_line, &format!("\x1b[31m   // {err}\x1b[0m"));

        let start_of_line = source.char_indices().filter_map(|(i, c)| if i < err.span.start && c == '\n' { Some(i) } else { None }).last().unwrap() + 1;

        source.insert_str(start_of_line, &format!("\x1b[31m"));
        source.insert_str(start_of_line + "\x1b[31m".len() + 3, &format!("\x1b[0m"));
    }

    println!();
    println!("{source}");
    println!("-----------------------------------------");
    println!();
    println!("{stream}");
    println!();
    println!("-----------------------------------------\x1b[31m");

    for err in errs {
        println!();
        println!("line {}:", source[0..err.span.start].chars().filter(|c| *c == '\n').count() + 1);
        println!("{}", err.to_string_multiline());
    }

    println!();
}