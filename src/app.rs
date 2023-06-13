use leptos::ev::SubmitEvent;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::api::{
    get_me, get_one_topic, get_topics, CreateAccessToken, CreateOptionInput, CreateTopic,
    CreateTopicInput, OAuth2PasswordRequest, Signup, SignupInput, Topic,
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
        <Link href="https://cdn.jsdelivr.net/npm/daisyui@3.1.0/dist/full.css" rel="stylesheet" />

        // content for this welcome page
        <Router>
            <main>
                <NavBar />
                <Routes>
                    <Route path="" view=|cx| view! { cx, <HomePage/> }/>
                    <Route path="/topic/create" view=|cx| view! { cx, <CreateTopicPage/> }/>
                    <Route path="/topic/:id" view=|cx| view! { cx, <TopicPage/> }/>
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
                        <a href="/" class="text-white font-semibold text-lg">"Voting System"</a>
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
    // FIXME: error handling
    let topics = move || topics.read(cx).map(|topics| topics.unwrap());
    let topics = move || match topics() {
        None => view! { cx, <p>"Loading..."</p> }.into_view(cx),
        Some(data) => data
            .into_iter()
            .map(|topic| {
                let (topic, _) = create_signal(cx, topic);
                view! { cx,
                    <TopicCard topic=topic show_action=true />
                }
                .into_view(cx)
            })
            .collect_view(cx),
    };

    let create_topic = move |_| {
        let goto = use_navigate(cx);
        // FIXME: error handling
        let _ = goto("/topic/create", NavigateOptions::default());
    };

    view! { cx,
        <div class="p-4">
            <button on:click=create_topic class="btn btn-primary my-4">"New Topic"</button>
            <Transition
                fallback=move || view! { cx, <p>"Loading..."</p>}
            >
                <div class="flex flex-col items-center w-full mx-auto">
                    {topics}
                </div>
            </Transition>
        </div>
    }
}

#[component]
fn TopicPage(cx: Scope) -> impl IntoView {
    let params = use_params_map(cx);
    let id = params.with(|params| params.get("id").unwrap().to_string());
    let topic = {
        let id = id.clone();
        create_resource(
            cx,
            || (),
            move |_| {
                let id = id.clone();
                async move { get_one_topic(id).await }
            },
        )
    };

    view! { cx,
        <div class="p-4 md:p-16 flex flex-col w-full item-center mx-auto">
            <ErrorBoundary
                fallback=move |cx, errors| view! { cx, <div>"Error"</div>}
            >
                <Transition
                    fallback=move || view! { cx, <p>"Loading..."</p>}
                >
                    {move || topic.read(cx).map(|topic| {
                        view! { cx,
                            {topic.map(|topic| {
                                let (topic, _) = create_signal(cx, topic);
                                // topic card
                                view! { cx,
                                    <TopicCard topic=topic show_action=false />
                                }
                                // options & votes

                                // comments
                            })}
                        }
                    })}
                </Transition>
            </ErrorBoundary>
        </div>
    }
}

#[component]
fn TopicCard(
    cx: Scope,
    #[prop(into)] topic: Signal<Topic>,
    #[prop(optional)] show_action: bool,
) -> impl IntoView {
    let topic = topic();
    let goto = use_navigate(cx);
    let open_topic = move |_| {
        // FIXME: error handling
        let _ = goto(&format!("/topic/{}", &topic.id), NavigateOptions::default());
    };
    let action = show_action.then(|| {
        view! { cx,
            <div class="card-actions justify-end">
            <button on:click=open_topic class="btn btn-primary">"Detail"</button>
            </div>
        }
    });
    view! { cx,
        <div class="card w-96 bg-base-200 mb-4 shadow-xl">
            <div class="card-body">
                <div class="text-3xl font-semibold">{topic.description}</div>
                <p>
                    "Starts at: "{topic.starts_at} <br />
                    "Ends at: "{topic.ends_at} <br />
                    "Updated at: "{topic.updated_at} <br />
                    "Stage: "{topic.stage} <br />
                    {action}
                </p>
            </div>
        </div>
    }
}

#[component]
fn CreateTopicPage(cx: Scope) -> impl IntoView {
    use leptos::html::Input;

    let description: NodeRef<Input> = create_node_ref(cx);
    let starts_at: NodeRef<Input> = create_node_ref(cx);
    let ends_at: NodeRef<Input> = create_node_ref(cx);

    let init_options = vec![(0, create_signal(cx, CreateOptionInput::default()))];
    let (options, set_options) = create_signal(cx, init_options);
    let mut next_id = 1;
    let add_option = move |_| {
        set_options.update(move |options| {
            options.push((next_id, create_signal(cx, CreateOptionInput::default())));
        });
        next_id += 1;
    };

    let create_topic = create_server_action::<CreateTopic>(cx);
    // TODO: redirect after submit
    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();

        let description = description().expect("<input> to exist").value();
        let starts_at = starts_at().expect("<input> to exist").value();
        let ends_at = ends_at().expect("<input> to exist").value();
        let options = options().into_iter().map(|(_, (opt, _))| opt()).collect();

        let input = CreateTopicInput {
            description,
            starts_at,
            ends_at,
            options,
        };

        create_topic.dispatch(CreateTopic { input });
    };

    let input_style = "w-full border-gray-300 rounded-md p-2";

    view! { cx,
        <div class="max-w-md mx-auto mt-8">
        <div class="bg-white rounded-lg shadow-md p-8">
            <h2 class="text-2xl font-semibold mb-6">"Create Topic"</h2>
            <form on:submit=on_submit>
                <div class="mb-4">
                    <label for="description" class="block text-gray-700 text-sm font-medium mb-2">"Description"</label>
                    <input type="text" id="description" name="description" node_ref=description class=input_style required />
                </div>
                <div class="mb-4">
                    <label for="starts_at" class="block text-gray-700 text-sm font-medium mb-2">"Starts At"</label>
                    <input type="datetime-local" id="starts_at" name="starts_at" node_ref=starts_at class=input_style required />
                </div>
                <div class="mb-4">
                    <label for="ends_at" class="block text-gray-700 text-sm font-medium mb-2">"Ends At"</label>
                    <input type="datetime-local" id="ends_at" name="ends_at" node_ref=ends_at class=input_style required />
                </div>
                <div>
                    <h3 class="text-lg font-semibold mb-2">"Options"</h3>
                    <div id="options-container">
                        <For
                            each=options
                            key=|option| option.0
                            view=move |cx, (_, (option, set_option))| {
                                let option = option();
                                view! { cx,
                                    <div class="mb-4">
                                        <input
                                            type="text"
                                            name="option-label[]"
                                            class=input_style
                                            placeholder="Option Label"
                                            prop:value=option.label
                                            on:input=move |ev| {
                                                set_option.update(|opt| {
                                                    opt.label = event_target_value(&ev);
                                                })
                                            }
                                            required
                                        />
                                        <textarea
                                            name="option-description[]"
                                            class=format!("{input_style} mt-2")
                                            placeholder="Option Description"
                                            on:input=move |ev| {
                                                set_option.update(|opt| {
                                                    opt.description = event_target_value(&ev);
                                                })
                                            }
                                            prop:value=option.description
                                        ></textarea>
                                    </div>
                                }
                            }
                        />
                    </div>
                    <button
                        type="button"
                        id="add-option"
                        class="bg-blue-500 text-white py-2 px-4 rounded-md"
                        on:click=add_option
                    >
                        "Add Option"
                    </button>
                </div>
                <div class="mt-6">
                    <button
                        type="submit"
                        class="bg-green-500 text-white py-2 px-4 rounded-md w-full"
                    >
                        "Create"
                    </button>
                </div>
            </form>
        </div>
        </div>
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

    let label_style = "block text-gray-700 text-sm font-medium mb-2";
    let input_style = "w-full border-gray-300 rounded-md p-2";

    view! { cx,
        <div class="max-w-md mx-auto mt-8">
            <div class="bg-white rounded-lg shadow-md p-8">
                <h2 class="text-2xl font-semibold mb-6">"Login"</h2>
                <form on:submit=on_submit>
                    <div class="mb-4">
                        <label for="username" class=label_style>"Username / Email"</label>
                        <input type="username" node_ref=username class=input_style required />
                    </div>
                    <div class="mb-6">
                        <label for="password" class=label_style>"Password"</label>
                        <input type="password" node_ref=password class=input_style required />
                    </div>
                    <div>
                        <button type="submit" class="bg-blue-500 text-white py-2 px-4 rounded-md w-full">"Login"</button>
                    </div>
                </form>
                <div class="mt-4 text-center">
                    <span class="text-gray-500">"Don't have an account? "</span>
                    <a href="/signup" class="text-blue-500 font-medium">"Signup"</a>
                </div>
            </div>
        </div>

        <ErrorBoundary
            // ref: https://leptos-rs.github.io/leptos/view/07_errors.html?highlight=error%20handling#errorboundary
            fallback=|cx, errors| view! { cx,
                <div class="m-2 border border-red-700 bg-red-400">
                    <p>"Login failed! Errors: "</p>
                    // we can render a list of errors as strings, if we'd like
                    <ul>
                        {move || errors.get()
                            .into_iter()
                            .map(|(_, e)| view! { cx, <li>{e.to_string()}</li>})
                            .collect_view(cx)
                        }
                    </ul>
                </div>
            }
        >
            {move || token().map(|token| token.map(|token| {
                state.update(|s| s.set_token(token.access_token));
                goto("/", NavigateOptions::default())
            }))}
        </ErrorBoundary>
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

    let label_style = "block text-gray-700 text-sm font-medium mb-2";
    let input_style = "w-full border-gray-300 rounded-md p-2";

    view! { cx,
        <div class="max-w-md mx-auto mt-8">
            <div class="bg-white rounded-lg shadow-md p-8">
                <h2 class="text-2xl font-semibold mb-6">"Signup"</h2>
                <form on:submit=on_submit>
                    <div class="mb-4">
                        <label for="username" class=label_style>"Username"</label>
                        <input type="username" node_ref=username class=input_style required />
                    </div>
                    <div class="mb-6">
                        <label for="email" class=label_style>"Email"</label>
                        <input type="email" node_ref=email class=input_style required />
                    </div>
                    <div class="mb-6">
                        <label for="password" class=label_style>"Password"</label>
                        <input type="password" node_ref=password class=input_style required />
                    </div>
                    <div>
                        <button type="submit" class="bg-blue-500 text-white py-2 px-4 rounded-md w-full">"Signup"</button>
                    </div>
                </form>
                <div class="mt-4 text-center">
                    <span class="text-gray-500">"Already have an account? "</span>
                    <a href="/login" class="text-blue-500 font-medium">"Login"</a>
                </div>
            </div>
        </div>

        <ErrorBoundary
            // ref: https://leptos-rs.github.io/leptos/view/07_errors.html?highlight=error%20handling#errorboundary
            fallback=|cx, errors| view! { cx,
                <div class="m-2 border border-red-700 bg-red-400">
                    <p>"Login failed! Errors: "</p>
                    // we can render a list of errors as strings, if we'd like
                    <ul>
                        {move || errors.get()
                            .into_iter()
                            .map(|(_, e)| view! { cx, <li>{e.to_string()}</li>})
                            .collect_view(cx)
                        }
                    </ul>
                </div>
            }
        >
            {move || signup_result().map(|r| if r.is_ok() {
                // FIXME: error handling
                let _ = goto("/login", NavigateOptions::default());
            })}
        </ErrorBoundary>
    }
}
