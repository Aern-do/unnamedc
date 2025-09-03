use unnamed_common::{IntoReport, Source, SourceCache};
use unnamed_lexer::Lexer;

fn main() {
    const SOURCE: Source = Source::new(
        r#"
        trait Add {
	        fn add(self, value: Self): Self;
        }

        func foo[N](lhs: N, rhs: N): N {
	        if condition() {
		        return bar(lhs, rhs);
	        }
    
	        lhs
        }

        func bar[N](lhs: N, rhs: N): N {
            if condition() {
                foo(lhs, rhs)
            } else {
                lhs + rhs
            }
        }

        func z[N](lhs: N): N {
            bar(lhs, N::default())
        }
    "#,
        "test.u",
    );

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
