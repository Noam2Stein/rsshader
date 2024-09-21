use std::fmt::Display;

use rsshader::{
    source::*,
    diagnostic::*,
    tokenization::*,
    parsing::*,
    syntax::*,
};

const SRC: &SrcFile = SrcFile::new(
    "
    mod rect;
    use
    "   
);

fn main() {
    let (output, errs) = {
        let mut errs = Vec::new();

        let output: String = {
            let mut tokenizer = tokenize(SRC);
            let ast = SyntaxTree::parse_tokens(&mut tokenizer, &mut errs);

            ast.with_src(SRC).to_string()
        };

        errs.sort();

        (output, errs)
    };
    
    let src = format_src(&errs);

    let errs = if errs.len() > 0 {
        Some(
            errs.into_iter().map(|err|
                format!("line {}:\n{}", line_label(span_line(err.span)), err.to_string_multiline())
            ).collect::<Box<[String]>>().join("\n\n")
        )
    }
    else {
        None
    };
    
    println!();
    println!("{src}");
    println!();
    println!("-----------------------------------------");
    println!();
    println!("{output}");
    println!();

    if let Some(errs) = errs {
        println!("\x1b[31m-----------------------------------------");
        println!();
        println!("{errs}");
        println!();
    }
}
fn format_src(errs: &[Error]) -> String {
    let mut err_spans = errs.iter().map(|err| err.span).collect::<Vec<Span>>();
    err_spans.sort();

    let mut index = 0;
    while index + 1 < err_spans.len() {
        let span = err_spans[index];
        let next_span = err_spans[index + 1];
        
        if span.intersects(&next_span) {
            err_spans[index] = Span::connect(&span, &next_span);
            err_spans.remove(index + 1);
        }
        else {
            index += 1;
        }
    }
    
    err_spans.reverse();
    let mut src = SRC.s().to_string();
    for span in err_spans {
        src.insert_str(span.end(), "\x1b[0m");
        src.insert_str(span.start(), "\x1b[31m");
    }

    let lines_strs = src.split("\n").collect::<Box<[&str]>>();
    let offset = lines_strs.iter().map(|line| line.char_indices().find(|(_, c)| *c != ' ').map(|(i, _)| i).unwrap_or(usize::MAX)).min().unwrap();
    (0..lines_strs.len()).map(|line| {
        let line_label = line_label(line);

        let line_str = lines_strs[line];
        let line_str = &line_str[offset.min(line_str.len())..line_str.len()];

        if errs.iter().any(|err| span_line(err.span) == line) {
            let mut output = format!("\x1b[31m{line_label: >3}\x1b[0m\t{line_str}");
            for err in errs.iter().filter(|err| span_line(err.span) == line) {
                output += &format!("   \x1b[31m// {err}\x1b[0m");
            }

            output
        }
        else {
            format!("{line_label: >3}\t{line_str}")
        }
    }).collect::<Box<[String]>>().join("\n")
}
fn span_line(span: Span) -> usize {
    SRC.s()[0..span.start()].chars().filter(|c| *c == '\n').count()
}
fn line_label(line: usize) -> impl Display {
    line + 1
}