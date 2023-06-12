use leptos::ev::SubmitEvent;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::api::{
    get_me, get_topics, CreateAccessToken, CreateOptionInput, CreateTopic, CreateTopicInput,
    OAuth2PasswordRequest, Signup, SignupInput,
};

use crate::state::GlobalState;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    provide_context(cx, create_rw_signal(cx, GlobalState::new(cx)));

    view! { cx,
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>
        // sets the document title
        <Title text="NTNU CSIE Online Voting System" />
        // Load tailwind css
        <Script src="https://cdn.tailwindcss.com"></Script>

        // content for this welcome page
        <Router>
            <main>
                <NavBar />
                <Routes>
                    <Route path="" view=|cx| view! { cx, <HomePage/> }/>
                    <Route path="/topic/create" view=|cx| view! { cx, <CreateTopicPage/> }/>
                    <Route path="/login" view=|cx| view! { cx, <LoginPage/> }/>
                    <Route path="/signup" view=|cx| view! { cx, <SignupPage/> }/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn NavBar(cx: Scope) -> impl IntoView {
    let link_style = "text-gray-300 hover:bg-gray-700 px-3 py-2 rounded-md text-sm font-medium";
    let state = expect_context::<RwSignal<GlobalState>>(cx);
    let user = create_resource(cx, state.read_only(), |state| async move {
        match state.token() {
            Some(token) => Some(get_me(token.to_string()).await),
            None => None,
        }
    });
    let username = move || user.read(cx).and_then(|u| u).map(|u| u.map(|u| u.username));

    view! { cx,
        <nav class="bg-gray-800">
            <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                <div class="flex items-center h-16">
                    <div class="flex items-center">
                        <a href="#" class="text-white font-semibold text-lg">"Voting System"</a>
                    </div>
                    <div class="hidden md:block">
                        <div class="ml-4 flex items-center space-x-4">
                            <a href="/topics" class=link_style>"Topics"</a>
                            <a href="/about" class=link_style>"About"</a>
                        </div>
                    </div>
                    <div class="flex flex-grow" />
                    <div class="flex">
                        <ErrorBoundary
                            // FIXME: error handling
                            fallback=move |cx, _errors| view! {cx, <a href="/login" class=link_style>"Login"</a> }>
                            {move || match username() {
                                Some(username) => {
                                    view! { cx,
                                        <div class="flex items-center">
                                            <span class="text-gray-300 text-sm pr-2">{username}</span>
                                        </div>
                                    }.into_view(cx)
                                }
                                None => {
                                    view! {cx, <a href="/login" class=link_style>"Login"</a> }
                                        .into_view(cx)
                                }
                            }}

                        </ErrorBoundary>
                    </div>
                </div>
            </div>
        </nav>
    }
}

/// Renders the home page. Which contains the topic list
#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    let topics = create_resource(cx, || (), |_| async move { get_topics().await });
    let topics = move || topics.read(cx).map(|topics| topics.unwrap());

    view! { cx,
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

#[component]
fn LoginPage(cx: Scope) -> impl IntoView {
    use leptos::html::Input;

    let username: NodeRef<Input> = create_node_ref(cx);
    let password: NodeRef<Input> = create_node_ref(cx);
    let goto = use_navigate(cx);
    let create_access_token = create_server_action::<CreateAccessToken>(cx);
    let token = create_access_token.value();
    let state = expect_context::<RwSignal<GlobalState>>(cx);

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();

        let username = username().expect("<input> to exist").value();
        let password = password().expect("<input> to exist").value();

        let input = OAuth2PasswordRequest { username, password };

        create_access_token.dispatch(CreateAccessToken { input });
    };

    view! { cx,
        <p>"Login"</p>
        <form on:submit=on_submit>
            <label for="username">"Username"</label>
            <input name="username" type="text" node_ref=username />
            <br />

            <label for="password">"Password"</label>
            <input name="password" type="password" node_ref=password />
            <br />

            <input type="submit" value="Login" />
        </form>

        {move || token().map(|token| match token {
            Ok(token) => {
                state.update(|s| s.set_token(token.access_token));
                // FIXME: handle navigate error
                _ = goto("/", NavigateOptions::default());
            }
            Err(_) => {}
        })}
    }
}

#[component]
fn SignupPage(cx: Scope) -> impl IntoView {
    use leptos::html::Input;

    let username: NodeRef<Input> = create_node_ref(cx);
    let password: NodeRef<Input> = create_node_ref(cx);
    let email: NodeRef<Input> = create_node_ref(cx);
    let goto = use_navigate(cx);
    let signup = create_server_action::<Signup>(cx);
    let signup_result = signup.value();

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();

        let username = username().expect("<input> to exist").value();
        let password = password().expect("<input> to exist").value();
        let email = email().expect("<input> to exist").value();

        let input = SignupInput {
            username,
            password,
            email,
        };

        signup.dispatch(Signup { input });
    };

    view! { cx,
        <p>"Signup"</p>
        <form on:submit=on_submit>
            <label for="username">"Username"</label>
            <input name="username" type="text" node_ref=username />
            <br />

            <label for="password">"Password"</label>
            <input name="password" type="password" node_ref=password />
            <br />

            <label for="email">"Email"</label>
            <input name="email" type="email" node_ref=email />
            <br />

            <input type="submit" value="Signup" />
        </form>

        {move || signup_result().map(|r| if r.is_ok() {
            // FIXME: error handling
            let _ = goto("/login", NavigateOptions::default());
        })}
    }
}
