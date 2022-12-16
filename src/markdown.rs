pub mod frame;

use pulldown_cmark::{escape, html, Event, Options, Parser};
use serde::{Deserialize, Serialize};
use yew_agent::{HandlerId, Public, Worker, WorkerLink};

pub struct Markdown {
    link: WorkerLink<Self>,
}

#[derive(Serialize, Deserialize)]
pub enum MarkdownInput {
    N(u32),
    Content(String),
}

#[derive(Serialize, Deserialize)]
pub enum MarkdownOutput {
    Value(u32),
    Html(String),
}

impl Worker for Markdown {
    type Input = MarkdownInput;
    type Message = ();
    type Output = MarkdownOutput;
    type Reach = Public<Self>;

    fn create(link: WorkerLink<Self>) -> Self { Self { link } }

    fn update(&mut self, _msg: Self::Message) {
        // no messaging
    }

    fn handle_input(&mut self, msg: Self::Input, id: HandlerId) {
        // this runs in a web worker
        // and does not block the main
        // browser thread!

        match msg {
            MarkdownInput::N(n) => {
                log::info!("{}", n);

                fn fib(n: u32) -> u32 {
                    if n <= 1 {
                        1
                    } else {
                        fib(n - 1) + fib(n - 2)
                    }
                }

                let output = Self::Output::Value(fib(n));

                self.link.respond(id, output);
            }
            MarkdownInput::Content(content) => {
                let mut options = Options::empty();
                options.insert(Options::ENABLE_FOOTNOTES);
                options.insert(Options::ENABLE_HEADING_ATTRIBUTES);
                options.insert(Options::ENABLE_SMART_PUNCTUATION);
                options.insert(Options::ENABLE_STRIKETHROUGH);
                options.insert(Options::ENABLE_TABLES);
                options.insert(Options::ENABLE_TASKLISTS);

                let parser = Parser::new_ext(&content, options).map(|event| match event {
                    Event::HardBreak => Event::HardBreak,
                    Event::SoftBreak => Event::HardBreak,
                    _ => event,
                });

                let mut html_output = String::new();
                html::push_html(&mut html_output, parser);

                self.link.respond(id, MarkdownOutput::Html(html_output));
            }
        }
    }

    fn name_of_resource() -> &'static str { "markdown.js" }
}
