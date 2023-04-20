mod document_view;

use semdoc::{Memory, SemDoc};
use web_sys::HtmlInputElement;
use yew::{html, Component, Html, NodeRef};

use crate::document_view::DocumentView;

struct Viewer {
    document: Option<SemDoc<Memory>>,
    input_ref: NodeRef,
}

enum ViewerMessage {
    LoadDocument,
    DocumentLoaded(Vec<u8>),
}

impl Component for Viewer {
    type Message = ViewerMessage;

    type Properties = ();

    fn create(ctx: &yew::Context<Self>) -> Self {
        Self {
            document: None,
            input_ref: NodeRef::default(),
        }
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ViewerMessage::LoadDocument => {
                let input_el = self.input_ref.cast::<HtmlInputElement>().unwrap();
                let file = input_el.files().unwrap().get(0).unwrap();

                ctx.link().send_future(async {
                    let bytes = gloo_file::futures::read_as_bytes(&gloo_file::File::from(file))
                        .await
                        .unwrap();
                    ViewerMessage::DocumentLoaded(bytes)
                });
            }
            ViewerMessage::DocumentLoaded(bytes) => {
                let doc = semdoc::SemDoc::from_bytes(&bytes).unwrap();
                self.document.replace(doc);
            }
        }
        true
    }

    fn view(&self, ctx: &yew::Context<Self>) -> Html {
        html! {
            <div id="content">
            <label>{"Document Upload"}</label>
            <input type="file" ref={self.input_ref.clone()} onchange={ctx.link().callback(|_| ViewerMessage::LoadDocument)} />

            if let Some(document) = self.document.clone() {
                <DocumentView {document}/>
            }
            </div>
        }
    }
}

fn main() {
    yew::Renderer::<Viewer>::new().render();
}
