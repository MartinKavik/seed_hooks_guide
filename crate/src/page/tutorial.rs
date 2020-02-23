use crate::{generated::css_classes::C, Msg, Page};
use comp_state::{
    do_once, topo, use_state, use_state_unique, ChangedState, CloneState, StateAccess,
};
use comp_state_seed_extras::{
    after_render, after_render_once, bind, get_html_element_by_id, StateAccessEventHandlers,
    UpdateElLocal,
};
use comrak::{markdown_to_html, ComrakOptions};
use wasm_bindgen::JsCast;

use seed::{prelude::*, *};
pub fn view() -> Node<Msg> {
    div![
        class!["container" C.h_screen],
        div![
            class![
                "header",
                C.shadow_xl,
                C.bg_gray_600,
                C.text_gray_200,
                C.flex,
                C.justify_end,
                C.content_center,
                C.items_center
            ],
            a![
                class![
                    C.h_full,
                    C.border_r_2,
                    C.py_2,
                    C.px_2,
                    C.mx_4,
                    C.hover__text_white,
                    C.border_gray_100,
                    C.hover__border_white
                ],
                "SEED HOOKS"
            ],
            a![
                class![
                    C.h_full,
                    C.border_r_2,
                    C.py_2,
                    C.px_2,
                    C.mx_4,
                    C.hover__text_white,
                    C.border_gray_100,
                    C.hover__border_white
                ],
                attrs!(At::Href => Page::Tutorial.to_href()),
                "TUTORIAL"
            ],
            a![
                class![
                    C.h_full,
                    C.border_r_2,
                    C.py_2,
                    C.px_2,
                    C.mx_4,
                    C.hover__text_white,
                    C.border_gray_100,
                    C.hover__border_white
                ],
                attrs!(At::Href => Page::ApiRef.to_href()),
                "API REFERENCE"
            ],
        ],
        div![
            class![
                "left",
                // C.w_1of4,
                C.bg_gray_700,
                C.text_gray_400,
                C.overflow_y_auto
            ],
            left_bar_content(),
        ],
        div![class!["main", C.overflow_y_auto], main_screen_content()],
    ]
}

fn left_bar_content() -> Node<Msg> {
    div![
        class![C.p_3],
        h1!["TUTORIAL"],
        hr![class![C.my_8 C.border_b_2 C.border_gray_200]],
        ul![
            li![a![
                class![
                    C.ml_2,
                    C.hover__text_gray_100,
                    C.border_b_2,
                    C.border_transparent,
                    C.hover__border_gray_300
                ],
                attrs![At::Href=>"tutorial#start_here"],
                "Start Here"
            ]],
            li![a![
                class![
                    C.ml_2,
                    C.hover__text_gray_100,
                    C.border_b_2,
                    C.border_transparent,
                    C.hover__border_gray_300
                ],
                attrs![At::Href=>"tutorial#step1"],
                "Step 1 - Design & Component Data Flow",
            ]],
            li![a![
                class![
                    C.ml_2,
                    C.hover__text_gray_100,
                    C.border_b_2,
                    C.border_transparent,
                    C.hover__border_gray_300
                ],
                attrs![At::Href=>"tutorial#step2"],
                "Step 2 - Markdown Processing",
            ]],
            li![a![
                class![
                    C.ml_2,
                    C.hover__text_gray_100,
                    C.border_b_2,
                    C.border_transparent,
                    C.hover__border_gray_300
                ],
                attrs![At::Href=>"tutorial#step3"],
                "Step 3 -  Prettifying the Output",
            ]],
            li![a![
                class![
                    C.ml_2,
                    C.hover__text_gray_100,
                    C.border_b_2,
                    C.border_transparent,
                    C.hover__border_gray_300
                ],
                attrs![At::Href=>"tutorial#step4"],
                "Step 4 - Message Submission",
            ]],
        ],
    ]
}

// lifted from the `console_log` example
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = Prism)]
    pub fn highlightAll();
    #[wasm_bindgen(js_namespace = Prism)]
    pub fn highlightElement(el: web_sys::HtmlElement);
    #[wasm_bindgen(js_namespace = Prism)]
    fn highlightAllUnder(el: web_sys::HtmlElement);
}

// Prism.highlightElement(element, async, callback)

#[topo::nested]
fn section_desc<T: Into<String>>(href_name: T, title: T, description: T) -> Vec<Node<Msg>> {
    let drop_type = use_state(crate::DropType::default);

    let mut opts = ComrakOptions::default();
    opts.github_pre_lang = true;

    let title = markdown_to_html(&title.into(), &opts);
    let description = markdown_to_html(&description.into(), &opts);

    let desc_el = use_state(ElRef::<web_sys::HtmlElement>::default);
    do_once(|| drop_type.update(|dt| dt.dropped = true));

    if drop_type.get().dropped {
        after_render(move |_| {
            if let Some(desc_el) = desc_el.get().get() {
                let code_children = desc_el.get_elements_by_tag_name("h3");

                for idx in 0..code_children.length() {
                    let code_el = code_children.item(idx).unwrap();
                    code_el.set_class_name("text-xl py-3 pt-4");
                }

                let code_children = desc_el.get_elements_by_tag_name("code");

                for idx in 0..code_children.length() {
                    let code_el = code_children.item(idx).unwrap();
                    code_el.set_class_name("language-rust");
                    highlightElement(code_el.dyn_into::<web_sys::HtmlElement>().unwrap());
                }
            }
        });
    }

    nodes![
        h2![
            class![C.m_3, C.text_2xl],
            a![attrs![At::Name=>href_name.into()], raw!(&title)]
        ],
        hr![class![C.my_8 C.border_b_2 C.border_gray_200]],
        div![
            el_ref(&desc_el.get()),
            class!["api-description" C.m_3 C.leading_relaxed],
            raw!(&description)
        ],
    ]
}

fn main_screen_content() -> Node<Msg> {
    div![
        section_desc(
            "start_here",
            "Start Here",
            r#"
# Getting Started

In this tutorial we will create a live markdown renderer component that can freely re-used as needed.

You can see the finished component example by clicking [here](/tutorial_example).

To assist checking progress in this tutorial please see here for the [final code](https://github.com/rebo/markdown-editor).

What we are going to do in this section is 

1. Install nightly rust
1. Clone Seed quickstart
1. Install TailwindCSS
1. Setup Seed Hooks

## Before we start 

Currently **Seed Hooks** only work on nightly rust, this is due to requiring the feature `TrackCaller` therefore it is 
important to install a recent nightly. **Furthermore as of 19th February there is a regression in nightly rust which prevents 
`js_sys` and therefore `Seed` from compiling.  A fix is in the pipeline and waiting to be merged into the next Rust nightly.**

Therefore to be safe use the verison.

```
rustup install nightly-2020-02-17
rustup default nightly-2020-02-17
```

## Download Quickstart

Clone the Seed basic quick start:

```
git clone https://github.com/seed-rs/seed-quickstart markdown-editor
cd markdown-editor
```

Check it compiles and serves correctly with 

```
cargo make build; cargo make serve
```

You can access the site from `http://localhost:8000`. This will display a simple button counter.

## CSS 

We will be using [TailwindCSS](https://tailwindcss.com) for our CSS. Therefore we need to set this up.  Create a `package.json` in the project root with
these contents:

```
// In package.json...
{
    "name": "markdown-editor",
    "version": "0.0.1"
}
```

Also add `/node_modules` to your `.gitignore` file. 

Next install TailwindCss with 

```
yarn add tailwindcss
```

Next create a `styles.css` in the project root with these contents: 

```
// In styles.css...

@tailwind base;

@tailwind components;

@tailwind utilities;
```

Now build the tailwindCSS with 

```
npx tailwindcss build styles.css -o output.css
```

and add this to the `index.html`

```
// In index.html...

<head>
    ...
    <link rel="stylesheet" type="text/css" href="output.css">
    ...
</head>
```

### Seed Hooks Setup

In order to enable **Seed Hooks** add the following to `Cargo.toml` in the `[dependencies]` section. 

```
// In Cargo.toml...

comp_state = "0.2.1"
comp_state_seed_extras = "0.0.8"
```

Next, Seed hooks rely on the nightly `TrackCaller` feature you need to add the `#![feature(track_caller)]` feature flag to the top of `lib.rs`.

Remove all existing `Model` and `Msg` fields/variants. You will also want to remove the match processing of `Msg` in your update function.

You should also glob import both `comp_state` and `comp_state_seed_extras` with 

```
// In in lib.rs...

use comp_state::*;
use comp_state_seed_extras::*;
```

The final bit of setup required is to add a root component to the seed view. This is acheived by annotating 
the main seed view function with `#[topo::nested]`.   For now replace the contents of the root view with a simple `div!`.

```
#[topo::nested]
fn view(_model: &Model) -> impl View<Msg> {
    div![]
}
```

This annotation means that the view function becomes part of the component hierarchy. Indeed this acts as the root compnent 
under which all other components are structured.

The final base `lib.rs` should be as per below :

```
// In in lib.rs...

#![feature(track_caller)]
use comp_state::*;
use comp_state_seed_extras::*;
use seed::{prelude::*, *};

#[derive(Default)]
struct Model {}

enum Msg {}

fn update(_msg: Msg, _model: &mut Model, _: &mut impl Orders<Msg>) {}

#[topo::nested]
fn view(_model: &Model) -> impl View<Msg> {
    div![]
}

#[wasm_bindgen(start)]
pub fn render() {
    App::builder(update, view).build_and_start();
}
```
"#
        ),
        section_desc(
            "step1",
            "Step 1 - Design & Component Data Flow",
            r#"
### Design

We want to create a reusable markdown renderer component. The only brief is that the user should be able to pass a message
to the component which sends the rendered html to Seed on submission.

What we are going to do in this section is 

1. Design the rough layout and styling for the component
1. Decide what state needs to be stored locally
1. Ensure state can be outputted into a preview div

Visually we want a split view, the left effectively being one big textbox, the right being a preview render of the markdown.
We also want a submit button on the button along the bottom. Something like this (for now we will ignore message requirement.)


```
// In in lib.rs...
#[topo::nested]
fn markdown_editor() -> Node<Msg> {
    div![
        class!["flex flex-col"],
        div![
            class!["flex flex-row"],
            div![class!["w-1/2"], "Markdown:"],
            div![class!["w-1/2"], "Preview:"],
        ],
        div![
            class!["flex flex-row h-64"],
            textarea![
                class!["bg-red-300 h-full flex-none w-1/2"],
                attrs![At::Type => "textbox"],
            ],
            div![
                class!["md-preview bg-yellow-300 h-full flex-none w-1/2"],
            ]
        ],
        div![
            class!["flex justify-end pt-2"],
            button![class!["bg-green-400 p-4 m-2"], "Submit"]
        ]
    ]
} 
```

This component can be rendered by calling it in the root view... 

```
#[topo::nested]
fn view(_model: &Model) -> impl View<Msg> {
    markdown_editor(Msg::SubmitMarkdownHtml)
}
```

The reason we annotate with `#[topo::nested]` is so that markdown_editor can operate as its own component with local state. 

### Cargo Watch

At this stage it would be worth setting up a cargo watch loop to rebuild the wasm file and re-serve so that you can see your changes
more immediately in the browser.

Run the following in separate terminal windows

```
cargo make serve
```
and 
```
cargo make watch
```

`cargo make serve` will ensure that your server is always running and `cargo make watch` will automatically re-compile the .wasm file. 
Therefore the only thing that you will need to do is refresh the browser after updating any of your rust files.



### Data Flow

So what do we want our component to do? When someone types in the text area we want the contents to be processed
by a markdown processor and the results viewable in the preview box on the right.

Ideally we would like the preview box to be in sync with the textarea cursor.

Datawise this means we need a **state variable** to store the current text area content, this then gets processed by a filter 
on `Input` event.  

In order to create this **state variable** we will use the first (and most used) hook function `use_state()`. Add the following
at the top of the function.

```
// In in lib.rs...
fn markdown_editor() -> Node<Msg> {
    let source = use_state(|| String::new());
    ...
```
This creates a **state variable** accessor `source` this accessor is used to get and set some `String` data associated with this component. 

Next we need to bind this source to the `value` attribute of the textarea. Modify the `textarea!` to include this a bind directive.

```
// In in lib.rs...
fn markdown_editor() -> Node<Msg> {
    ...
    textarea![
        bind(At::Value, source),
        ...
]
```

You will get an error that `Msg` does not implement `Default`. The reason for this error is that currently all Seed `EventHandler`s need to
return a Msg to the Seed app. This requirement will be lifted in the future.  To fix this add a default implementation to the `Msg` enum.

```
// In in lib.rs...

enum Msg {
    NoOp,
}

impl std::default::Default for Msg {
    fn default() -> Self {
        Msg::NoOp
    }
}
```
If you still have a `match msg` in your `update()` function you will need to add this variant to the match. i.e.

```

fn update(msg: Msg, _model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::NoOp => {}
    }
}
```

Lastly lets ensure that the bind is working correctly and we will output the raw textarea input into the preview div:

```
// In in lib.rs...
fn markdown_editor() -> Node<Msg> {
    ...

    div![
        class!["md-preview bg-yellow-300 flex-none w-1/2"],
        source.get()
    ]
    ...
```
Refreshing your browser now (`https://localhost:8000) and typing in the text area should out put the text directly within the preview `div`.

"#
        ),
        section_desc(
            "step2",
            "Step 2 - Markdown Processing",
            r#"

What we are going to do in this section is 

1. Process the source state variable as markdown prior to rendering in the view.

### How to process

We now have a basic bind set up, updating the text area will update the `source` state variable.
This is then directly output to the preview div.

Instead of outputting directly to the preview div, we want it to be processed as markdown. 
Fortunately Seed has an in-built macro that renders markdown from a `&str`.

Simply wrap `source.get()` in `md![&source.get()` in the preview div:

```
// In in lib.rs...

div![
    class!["md-preview bg-yellow-300 flex-none w-1/2"],
    md![&source.get()]
]
```

Here is the final `markdown_editor` function at this stage. 

```
// In in lib.rs...
#[topo::nested]
fn markdown_editor() -> Node<Msg> {
    let source = use_state(|| String::new());

    div![
        class!["flex flex-col"],
        div![
            class!["flex flex-row h-40"],
            textarea![
                bind(At::Value, source),
                class!["bg-red-300 h-full flex-none w-1/2"],
                attrs![At::Type => "textbox"],
            ],
            div![
                class!["md-preview bg-yellow-300 h-full flex-none w-1/2"],
                md![&source.get()]
            ]
        ],
        div![
            class!["flex justify-end pt-2"],
            button![class!["bg-green-400 p-4 m-2"], "Submit"]
        ]
    ]
}
```
"#
        ),
        section_desc(
            "step3",
            "Step 3 - Prettifying the output",
            r#"

What we are going to do in this section is 

1. Improve the styling of the component
1. Learn how to use `ElRef`s to programatically access the dom.
1. Add auto scrolling so the preview matches the textarea scroll location 

### Preview & Textarea div

The UI currently is functional but it can be improved, specifically regarding the preview render.

We therefore need to style both to better improve the UI.  TailwindCSS by default does a normalise
pass on all styles. 

We will use `github-markdown-css` for this, download the [markdown css file](https://raw.githubusercontent.com/sindresorhus/github-markdown-css/gh-pages/github-markdown.css)
and place it in the project root. Next link to this stylesheet in `index.html`.

```
// in index.html
... 
<head>
    ...
    <link rel="stylesheet" type="text/css" href="output.css">
    <link rel="stylesheet" href="github-markdown.css"> 
    .... 
```

all the class `markdown-body` to the preview div.

```
/// in lib.rs
div![
    class!["md-preview markdown-body"],
```
This will ensure the preview pane's markdown is rendered correctly.

Furthermore we woud like the `textarea` input to be mono-space. Therefore adjust it's class:

```
// In in lib.rs...

textarea![
    ...
    class!["font-mono p-2 h-full flex-none w-1/2 border-gray-200 border shadow-lg"],
    ...
],
```            

Finally lets improve the look of the preview pane. Here I have used two `class!` macros to separate the 
TailwindCss classes from specificly chosen classes.

```
div![
    class!["md-preview markdown-body"],
    class!["overflow-auto p-2 pl-4 h-full flex-none w-1/2 border-gray-200 bg-indigo-100 border shadow-lg"],
    ...
```

Lets try how it all works now, save the file refresh the browser. Try typing the following into the text area: 

```
# Seed Rocks

**Yes** indeed it does *rock*.

```

### Auto scrolling the preview

When we edit the text area we ideally would like the preview to scroll to a similar position. 
This would enable our edits to be easier to see. Therefore we want to programatically scroll the md-preview div 
on `KeyUp` and also on `Scroll` events.

In order to do this we need to identify the md-preview and also the textarea with ElRefs. 
These are Seed's way of identifying individual elements. 

Due to the fact that we are going to refer to specific html elements via `web_sys` we need to add that as a dependency.

In `Cargo.toml` add the following to the dependencies section: 

```
[dependencies]
...
web-sys = "^0.3.32"
...
```

and enable access to the following types at the top of `lib.rs`:

```
use web_sys::{HtmlElement, HtmlTextAreaElement};
```

after the `let source = use_state..` line add two more use_state hooks. 

```
// In in lib.rs...

let preview_el = use_state::<ElRef<HtmlElement>, _>(ElRef::default);
let textarea_el = use_state::<ElRef<HtmlTextAreaElement>, _>(ElRef::default);
```

This provides access to two el_refs which we can later associate with specific elements. 

In order to do this we use the `el_ref()` function within the respective elements...

```
// In in lib.rs...

textarea![
    el_ref(&textarea_el.get()),
    ...
```

and 

```
// In in lib.rs...

div![
    class!["md-preview"],
    el_ref(&preview_el.get()),
    ...
```

In order to set the respective scroll on the preview we use a simple percentage of textarea
scroll as a guide.

This is achieved via the following event handler:  

```
// In in lib.rs...

textarea_el.input_ev(Ev::KeyUp, move |el, _| {
    if let (Some(textarea), Some(preview)) = (el.get(), preview_el.get().get()) {
        let scroll_percentage = (textarea.scroll_top() as f64) / (textarea.scroll_height() as f64);
        let new_scroll_top = (preview.scroll_height() as f64) * scroll_percentage;
        preview.set_scroll_top(new_scroll_top as i32);
    }
}),
```

We also add an identical EventHandler callback for an `Ev::Scroll` event.

Once all the above is completed. scrolling and cursor navigating through the textarea will 
result in a corrsponding scroll of the preview div.
"#
        ),
        section_desc(
            "step4",
            "Step 4 - Message Submission",
            r#"
The final step is to modify the function signature to allow an arbiatry message to be passed.
This message will then be sent to seed on pressing of the submit button. 

The message should permit a String argument. Hence we will use the following: 

```
// In in lib.rs...

fn markdown_editor(msg_handler: impl FnOnce(String) -> Ms + 'static + Clone) -> Node<Msg>
```

We modify the `Msg` type to allow for one of these. 

```
// In in lib.rs...

enum Msg {
    NoOp,
    SubmitMarkdownHtml(String),
}
```

Finally we add an `Ev::Click` event handler to the submit button.

```
// In in lib.rs...

button![
    class!["bg-green-400 p-4 m-2"],
    "Submit",
    mouse_ev(Ev::Click, move |_| msg_handler(processed_md))
]
```

and handle it in the `update()` function :

```
// In in lib.rs...

fn update(msg: Msg, _model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::SubmitMarkdownHtml(html) => log!(html),
        Msg::NoOp => {}
    }
} 
```

So finally when the form button is clicked, an output of the processed html will be
logged to the console from the Seed update function.

### Final Thoughts

Overall the component fufills the brief, obviously there are visual areas for improvement.  Performance-wise there are some
considerations. In a heavy page (such as this one) where the virtual dom is completely re-diffed every update
the component may appear sluggish. 

Some ways to deal with this situation including making use of Seed's `Keyed` updates to limit dom patching to a specific element
or rendering the markdown directly to the dom in an `after_render` callback.

The final `lib.rs` file is below:

```
#![feature(track_caller)]
use comp_state::*;
use comp_state_seed_extras::*;
use comrak::{markdown_to_html, ComrakOptions};
use seed::{prelude::*, *};

#[derive(Default)]
struct Model {}

enum Msg {
    NoOp,
    SubmitMarkdownHtml(String),
}

impl std::default::Default for Msg {
    fn default() -> Self {
        Msg::NoOp
    }
}

fn update(msg: Msg, _model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::SubmitMarkdownHtml(html) => log!(html),
        Msg::NoOp => {}
    }
}

#[topo::nested]
fn view(_model: &Model) -> impl View<Msg> {
    markdown_editor(Msg::SubmitMarkdownHtml)
}

fn set_scroll(textarea: HtmlTextAreaElement, preview: HtmlElement) {
    let scroll_percentage = (textarea.scroll_top() as f64) / (textarea.scroll_height() as f64);
    let new_scroll_top = (preview.scroll_height() as f64) * scroll_percentage;
    preview.set_scroll_top(new_scroll_top as i32);
}

#[topo::nested]
fn markdown_editor(msg_handler: impl FnOnce(String) -> Msg + 'static + Clone) -> Node<Msg> {
    use web_sys::{HtmlElement, HtmlTextAreaElement};

    let source = use_state(|| String::new());
    let preview_el = use_state::<ElRef<HtmlElement>, _>(ElRef::default);
    let textarea_el = use_state::<ElRef<HtmlTextAreaElement>, _>(ElRef::default);

    let processed_md = markdown_to_html(&source.get(), &ComrakOptions::default());

    div![
        class!["flex flex-col"],
        div![
            class!["flex flex-row"],
            div![class!["w-1/2"], "Markdown:"],
            div![class!["w-1/2"], "Preview:"],
        ],
        div![
            class!["flex flex-row h-64"],
            textarea![
                el_ref(&textarea_el.get()),
                bind(At::Value, source),
                class!["font-mono p-2 h-full flex-none w-1/2 border-gray-200 border shadow-lg"],
                attrs![At::Type => "textbox"],
                textarea_el.input_ev(Ev::KeyUp, move |el, _| {
                    if let (Some(textarea), Some(preview)) = (el.get(), preview_el.get().get()) {
                        set_scroll(textarea, preview);
                    }
                }),
                textarea_el.input_ev(Ev::Scroll, move |el, _| {
                    if let (Some(textarea), Some(preview)) = (el.get(), preview_el.get().get()) {
                        set_scroll(textarea, preview);
                    }
                })
            ],
            div![
                class!["md-preview markdown-body"],
                el_ref(&preview_el.get()),
                class!["overflow-auto p-2 pl-4 h-full flex-none w-1/2 border-gray-200 bg-indigo-100 border shadow-lg"],
                md![source.get()]
            ]
        ],
        div![
            class!["flex justify-end pt-2"],
            button![
                class!["bg-green-400 rounded-lg p-4 m-2"],
                "Submit",
                mouse_ev(Ev::Click, move |_| msg_handler(processed_md))
            ]
        ]
    ]
}

#[wasm_bindgen(start)]
pub fn render() {
    App::builder(update, view).build_and_start();
}

```
    "#
        ),
    ]
}
