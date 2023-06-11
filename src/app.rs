use leptos::ev::SubmitEvent;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::api::{get_topics, CreateOptionInput, CreateTopic, CreateTopicInput};

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! { cx,
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>

        // sets the document title
        <Title text="NTNU CSIE Online Voting System" />

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=|cx| view! { cx, <HomePage/> }/>
                    <Route path="/topic/create" view=|cx| view! { cx, <CreateTopicPage/> }/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page. Which contains the topic list
#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    let topics = create_resource(cx, || (), |_| async move { get_topics().await });
    let topics = move || topics.read(cx).map(|topics| topics.unwrap());

    view! { cx,
        <h1>"Welcome to NTNU CSIE Online Voting System"</h1>
        <Transition
            fallback=move || view! { cx, <p>"Loading..."</p>}
        >
            {match topics() {
                None => {
                    view! { cx, <p>"Loading..."</p> }
                        .into_view(cx)
                },
                Some(data) => {
                    data.into_iter().map(|topic| {
                        view! { cx, <p>{topic.id}</p> }
                            .into_view(cx)
                    }).collect_view(cx)
                }
            }}
        </Transition>
    }
}

// TODO: add option input
#[component]
fn CreateTopicPage(cx: Scope) -> impl IntoView {
    use leptos::html::Input;

    let description: NodeRef<Input> = create_node_ref(cx);
    let starts_at: NodeRef<Input> = create_node_ref(cx);
    let ends_at: NodeRef<Input> = create_node_ref(cx);

    let create_topic_action = create_server_action::<CreateTopic>(cx);

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();

        let description = description().expect("<input> to exist").value();
        let starts_at = starts_at().expect("<input> to exist").value();
        let ends_at = ends_at().expect("<input> to exist").value();

        let input = CreateTopicInput {
            description,
            starts_at,
            ends_at,
            options: vec![CreateOptionInput {
                label: "Option 0".to_string(),
                description: "This is the first option.".to_string(),
            }],
        };

        create_topic_action.dispatch(CreateTopic { input });
    };

    view! { cx,
        <p>"Create Topic"</p>
        <form on:submit=on_submit>
            <label for="description">"Description"</label>
            <input name="description" type="text" node_ref=description />
            <br />

            <label for="starts_at">"Starts at"</label>
            <input name="starts_at" type="datetime-local" node_ref=starts_at />
            <br />

            <label for="ends_at">"Ends at"</label>
            <input name="ends_at" type="datetime-local" node_ref=ends_at />
            <br />

            <ul>
            </ul>

            <input type="submit" value="Submit!" />
        </form>
    }
}
