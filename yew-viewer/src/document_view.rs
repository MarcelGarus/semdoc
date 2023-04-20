use semdoc::{Block, Memory, SemDoc};
use yew::{function_component, html, Html, Properties};

#[derive(Clone, PartialEq, Properties)]
pub struct DocumentViewProps {
    pub document: SemDoc<Memory>,
}

#[derive(Clone, Copy, Default)]
struct RenderOptions {
    is_heading: bool,
    heading_level: u8,
    is_list_element: bool,
}

fn block_to_html(block: &Block<Memory>, options: RenderOptions) -> Html {
    match block {
        Block::Error(_) => html! {<p>{"Error"}</p>},
        Block::Empty => html! {},
        Block::Text(text) => {
            if options.is_heading {
                match options.heading_level {
                    1 => html! {<h1>{text}</h1>},
                    2 => html! {<h2>{text}</h2>},
                    3 => html! {<h3>{text}</h3>},
                    4 => html! {<h4>{text}</h4>},
                    5 => html! {<h5>{text}</h5>},
                    6 => html! {<h6>{text}</h6>},
                    _ => html! {<p>{text}</p>},
                }
            } else {
                html! {<p>{text}</p>}
            }
        }
        Block::Section { title, body } => {
            let mut title_options = options;
            title_options.is_heading = true;
            title_options.heading_level += 1;
            html! {
            <section>
                {block_to_html(title, title_options)}
                {block_to_html(body, options)}
            </section>}
        }
        Block::Flow(flow) => html! {
            {flow.iter().map(|block| block_to_html(block, options)).collect::<Html>()}
        },
        Block::Paragraphs(paragraphs) => html! {
            {paragraphs.iter().map(|par| html!{
                <>
                {block_to_html(par, options)}

                </>
                }).collect::<Html>()}
        },
        Block::BulletList(list) => {
            let mut list_options = options;
            list_options.is_list_element = true;
            html! { <ul>
                {list.iter().map(|item| html! {<li>{block_to_html(item, list_options)}</li>}).collect::<Html>()}
            </ul>}
        }
        Block::OrderedList(list) => html! { <ul>
            {list.iter().map(|item| html! {<li>{block_to_html(item, options)}</li>}).collect::<Html>()}
        </ul>},
    }
}

#[function_component]
pub fn DocumentView(props: &DocumentViewProps) -> Html {
    let document = props.document.clone();

    block_to_html(&document.block, RenderOptions::default())
}
