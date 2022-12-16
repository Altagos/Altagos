use std::rc::Rc;

use gloo::storage::{LocalStorage, Storage};
use log::{debug, error, trace};
use web_sys::HtmlTextAreaElement;
use yew::{
    events::{FocusEvent, KeyboardEvent},
    html::Scope,
    prelude::*,
    virtual_dom::VNode,
};
use yew_agent::{Bridge, Bridged};

use crate::markdown::{Markdown, MarkdownInput, MarkdownOutput};

const KEY: &str = "me.altagos.markdown_frame";

pub struct MarkdownFrame {
    pub markdown: String,
    pub content: String,
    pub loading: bool,
    pub worker: Box<dyn Bridge<Markdown>>,
}

pub enum MarkdownFrameMsg {
    Render(String),
    MarkdownWorker(MarkdownOutput),
}

#[derive(Properties, PartialEq)]
pub struct MarkdownFrameProps {
    #[prop_or("".to_string())]
    pub content: String,
}

impl Component for MarkdownFrame {
    type Message = MarkdownFrameMsg;
    type Properties = MarkdownFrameProps;

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            MarkdownFrameMsg::Render(md) => {
                self.markdown = md.clone();
                self.worker.send(MarkdownInput::Content(md));
                if let Err(e) = LocalStorage::set(KEY, self.markdown.clone()) {
                    error!("Error saving markdown frame to local storage: {e}");
                }
            }
            MarkdownFrameMsg::MarkdownWorker(msg) => match msg {
                MarkdownOutput::Value(_) => {}
                MarkdownOutput::Html(content) => {
                    self.content = content;
                    self.loading = false;
                }
            },
        }

        true
    }

    fn create(ctx: &Context<Self>) -> Self {
        let cb = {
            let link = ctx.link().clone();
            move |e| link.send_message(MarkdownFrameMsg::MarkdownWorker(e))
        };
        let mut markdown = Markdown::bridge(Rc::new(cb));

        let props = ctx.props();
        let mut md_content = props.content.clone();

        if &md_content == "" {
            md_content = LocalStorage::get(KEY).unwrap_or(md_content);
        }

        markdown.send(MarkdownInput::Content(md_content.clone()));

        Self {
            markdown: md_content,
            content: "".to_string(),
            loading: true,
            worker: markdown,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if self.loading {
            self.view_loading(ctx)
        } else {
            self.view_content(ctx)
        }
    }

    fn destroy(&mut self, ctx: &Context<Self>) {
        if let Err(e) = LocalStorage::set(KEY, self.markdown.clone()) {
            error!("Error saving markdown frame to local storage: {e}");
        }
    }
}

impl MarkdownFrame {
    fn view_loading(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="markdown-frame">
                <div class="markdown-input">{ self.view_input(ctx.link()) }</div>
                <div class="markdown-output">
                    { "Loading..." }
                </div>
            </div>
        }
    }

    fn view_content(&self, ctx: &Context<Self>) -> Html {
        let content = self.content.clone();

        html! {
            <div class="markdown-frame">
                <div class="markdown-input">{ self.view_input(ctx.link()) }</div>
                <div class="markdown-output">
                    <SafeHtml html={content} />
                </div>
            </div>
        }
    }

    fn view_input(&self, link: &Scope<Self>) -> Html {
        let oninput = link.callback(|e: InputEvent| {
            let input: HtmlTextAreaElement = e.target_unchecked_into();
            let value = input.value();

            MarkdownFrameMsg::Render(value)
        });

        let content = self.markdown.clone();

        html! {
            <textarea
                class="markdown"
                value={content}
                {oninput}
            >
            { content }
            </textarea>
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub html: String,
}

#[function_component(SafeHtml)]
pub fn safe_html(props: &Props) -> Html {
    let div = gloo::utils::document().create_element("div").unwrap();
    div.set_inner_html(&props.html.clone());

    Html::VRef(div.into())
}
