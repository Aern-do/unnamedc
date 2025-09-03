use unnamed_common::{IntoReport, Source, SourceCache};
use unnamed_lexer::Lexer;

fn main() {
    const SOURCE: Source = Source::new("0xff 120 0o11 0b101 fdf", "test.u");

    let mut source_cache = SourceCache::new();
    source_cache.insert(SOURCE);

    let lexer = Lexer::new(SOURCE);

    for token in lexer {
        match token {
            Ok(token) => println!("{token:?}"),
            Err(error) => {
                let report = error.into_report(SOURCE);
                report
                    .eprint(&mut source_cache)
                    .expect("failed to print report");

                break;
            }
        }
    }
}
