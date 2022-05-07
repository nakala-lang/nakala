use miette::Result;
use parser::{parse, source::Source};
use reedline::{DefaultPrompt, Reedline, Signal};

fn main() -> Result<()> {
    let mut line_editor = Reedline::create();
    let prompt = DefaultPrompt::default();

    loop {
        let sig = line_editor.read_line(&prompt).unwrap();
        match sig {
            Signal::Success(buffer) => {
                let source = Source::new(&buffer, "stdin".to_string());
                let parse = parse(source)?;
                println!("{:#?}", parse.stmts);
            }
            Signal::CtrlD | Signal::CtrlC => {
                println!("\nAborted!");
                break Ok(());
            }
        }
    }
}
