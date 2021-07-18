use clap::ArgMatches;
use crossterm::cursor::{MoveLeft, MoveToColumn};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers, MouseEvent};
use crossterm::style::{style, Attribute, Color, Print, PrintStyledContent};
use crossterm::QueueableCommand;
use crossterm::Result;
use engine::env::Env;
use std::io::{self, Stdout, Write};

use crate::NakalaResult;

pub struct LineEditor {
    stdout: Stdout,
    buffer: String,
    env: Env,
    prompt: &'static str,
    show_parse: bool,
    show_hir: bool,
}

impl LineEditor {
    pub fn new(prompt: &'static str, cli_args: ArgMatches) -> Result<Self> {
        let mut le = Self {
            stdout: io::stdout(),
            buffer: String::new(),
            env: Env::default(),
            prompt,
            show_parse: cli_args.is_present("parse"),
            show_hir: cli_args.is_present("hir"),
        };

        le.print_version_message()?;
        le.reset_prompt()?;

        Ok(le)
    }

    pub fn dispatch_key_event(&mut self, key_event: KeyEvent) -> Result<()> {
        if key_event == KILL_KEY_EVENT {
            self.exit();
        }

        let KeyEvent { code, .. } = key_event;
        match code {
            KeyCode::Char(c) => {
                self.buffer.push(c);
                self.stdout.queue(Print(c))?.flush()?;
            }
            KeyCode::Backspace => {
                if !self.buffer.is_empty() {
                    self.buffer.pop();

                    self.stdout
                        .queue(MoveLeft(1))?
                        .queue(Print(' '))?
                        .queue(MoveLeft(1))?
                        .flush()?;
                }
            }
            KeyCode::Enter => {
                self.new_line()?;
                self.execute_buffer()?;
                self.reset_prompt()?;
            }
            _ => {}
        }

        Ok(())
    }

    fn parse_buffer(&mut self) {
        match super::parse_and_eval_buffer(&self.buffer, &mut self.env) {
            Ok(NakalaResult { parse, hir, val }) => {
                if self.show_parse {
                    self.print_big_string(parse.debug_tree())
                        .expect("Failed to print big string");
                }

                if self.show_hir {
                    self.print_big_string(format!("{:#?}", hir.stmts))
                        .expect("Failed to print big string");
                }

                print!("{:?}", val);
            }
            Err(err) => {
                if self.print_error(Box::new(err)).is_err() {
                    self.exit();
                }
            }
        }
    }

    #[allow(unused_variables)]
    pub fn dispatch_mouse_event(&mut self, mouse_event: MouseEvent) -> Result<()> {
        Ok(())
    }

    #[allow(unused_variables)]
    pub fn dispatch_resize_event(&mut self, new_cols: u16, new_rows: u16) -> Result<()> {
        Ok(())
    }

    fn execute_buffer(&mut self) -> Result<()> {
        match self.buffer.as_str() {
            "help" => self.print_help_message()?,
            "version" => self.print_version_message()?,
            "__env" => self.print_env()?,
            _ => self.parse_buffer(),
        }

        self.buffer.clear();

        Ok(())
    }

    fn print_env(&mut self) -> Result<()> {
        let mut env_str = String::new();
        let (vars, funcs) = self.env.get_all_bindings();
        for (k, v) in vars {
            env_str.push_str(format!("`{:?}`: `{:?}`\n", k, v).as_str());
        }

        for (k, v) in funcs {
            env_str.push_str(format!("`{:?}`: `{:?}`\n", k, v).as_str());
        }

        self.print_big_string(env_str)
    }

    fn print_error(&mut self, err: Box<dyn std::error::Error>) -> Result<()> {
        let error_header = style("ERROR: ").with(Color::Red).attribute(Attribute::Bold);
        self.stdout
            .queue(PrintStyledContent(error_header))?
            .queue(Print(err.to_string()))?
            .flush()?;

        self.new_line()?;

        Ok(())
    }

    fn new_line(&mut self) -> Result<()> {
        self.stdout
            .queue(Print('\n'))?
            .queue(MoveToColumn(0))?
            .flush()?;

        Ok(())
    }

    fn reset_prompt(&mut self) -> Result<()> {
        self.new_line()?;
        self.stdout.queue(Print(self.prompt))?.flush()?;

        Ok(())
    }

    fn print_version_message(&mut self) -> Result<()> {
        let header_msg = format!("Nakala v{}\n", env!("CARGO_PKG_VERSION"));
        let body_msg = "Type \"help\" for more information";

        let styled_header = style(header_msg)
            .with(Color::DarkYellow)
            .attribute(Attribute::Bold);
        self.stdout
            .queue(PrintStyledContent(styled_header))?
            .queue(MoveToColumn(0))?
            .queue(Print(body_msg))?
            .flush()?;

        Ok(())
    }

    fn print_help_message(&mut self) -> Result<()> {
        let title = "Nakala";
        let header_msg = " - a hobby programming language based on azrg's Eldiro blog posts.\n";
        let body_msg = "For more information, visit https://github.com/reaganmcf/nakala";

        let styled_title = style(title)
            .with(Color::DarkYellow)
            .attribute(Attribute::Bold);

        self.stdout
            .queue(PrintStyledContent(styled_title))?
            .queue(Print(header_msg))?
            .queue(MoveToColumn(0))?
            .queue(Print(body_msg))?
            .flush()?;

        Ok(())
    }

    fn print_big_string(&mut self, big_string: String) -> Result<()> {
        for chunk in big_string.split('\n') {
            self.stdout.queue(Print(chunk))?;
            self.new_line()?;
        }

        Ok(())
    }

    fn exit(&self) {
        // Cleanup
        crossterm::terminal::disable_raw_mode().expect("Failed to disable raw mode");
        std::process::exit(0);
    }
}

const KILL_KEY_EVENT: KeyEvent = KeyEvent {
    code: KeyCode::Char('c'),
    modifiers: KeyModifiers::CONTROL,
};
