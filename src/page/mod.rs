pub mod topic;

pub use topic::*;

use leptos::ev::SubmitEvent;
use leptos::*;
use leptos_router::*;

use crate::api::{get_topics, CreateAccessToken, OAuth2PasswordRequest, Signup, SignupInput};
use crate::component::*;
use crate::state::GlobalState;

/// Renders the home page. Which contains the topic list
#[component]
pub fn HomePage(cx: Scope) -> impl IntoView {
    let loading =
        move || view! { cx, <p>"Loading..." <span class="loading loading-spinner"></span></p> };
    let topics = create_local_resource(cx, || (), |_| async move { get_topics().await });
    let topics = move || {
        // FIXME: error handling
        match topics.read(cx).map(|topics| topics.unwrap()) {
            None => loading().into_view(cx),
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
        }
    };

    view! { cx,
        <div class="p-4">
            <button
                on:click=move |_| {
                    let goto = use_navigate(cx);
                    // FIXME: error handling
                    let _ = goto("/topic/create", NavigateOptions::default());
                }
                class="btn btn-primary my-4"
            >"New Topic"</button>
            <Transition fallback=loading>
                <div class="flex flex-col items-center w-full mx-auto">
                    {topics}
                </div>
            </Transition>
        </div>
    }
}

#[component]
pub fn LoginPage(cx: Scope) -> impl IntoView {
    use leptos::html::Input;

    let username: NodeRef<Input> = create_node_ref(cx);
    let password: NodeRef<Input> = create_node_ref(cx);
    let goto = use_navigate(cx);
    let create_access_token = create_server_action::<CreateAccessToken>(cx);
    let login_pending = create_access_token.pending();
    let token = create_access_token.value();
    let state = expect_context::<RwSignal<GlobalState>>(cx);

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();

        if login_pending() {
            return;
        }

        let username = username().expect("<input> to exist").value();
        let password = password().expect("<input> to exist").value();
        let input = OAuth2PasswordRequest { username, password };

        create_access_token.dispatch(CreateAccessToken { input });
    };

    let input_style = "input input-bordered input-info w-full max-w-md";

    view! { cx,
        <div class="max-w-md mx-auto mt-8">
            <div class="rounded-lg shadow-md p-8">
                <h2 class="text-2xl font-semibold mb-6">"Login"</h2>
                <form on:submit=on_submit>
                    <div class="mb-4">
                        <label for="username">
                            <div class="label-text">"Username / Email"</div>
                        </label>
                        <input id="username" type="username" node_ref=username class=input_style required />
                    </div>
                    <div class="mb-6">
                        <label for="password">
                            <div class="label-text">"Password"</div>
                        </label>
                        <input id="password" type="password" node_ref=password class=input_style required />
                    </div>
                    <div>
                        <button type="submit" class="btn btn-primary py-2 px-4 w-full">
                            {move || if login_pending() {
                                view! { cx, <span class="loading loading-spinner"></span> }.into_view(cx)
                            } else {
                                view! { cx, "Login" }.into_view(cx)
                            }}
                        </button>
                    </div>
                </form>
                <div class="mt-4 text-center">
                    <span>"Don't have an account? "</span>
                    <a href="/signup" class="link link-info">"Signup"</a>
                </div>
            </div>
        </div>

        <ErrorList error_title="Login failed!".to_string()>
            {move || token().map(|token| token.map(|token| {
                state.update(|s| s.set_token(token.access_token));
                // FIXME: error handling
                goto("/", NavigateOptions::default())
            }))}
        </ErrorList>
    }
}

#[component]
pub fn SignupPage(cx: Scope) -> impl IntoView {
    use leptos::html::Input;

    let username: NodeRef<Input> = create_node_ref(cx);
    let password: NodeRef<Input> = create_node_ref(cx);
    let email: NodeRef<Input> = create_node_ref(cx);
    let signup = create_server_action::<Signup>(cx);
    let signup_result = signup.value();
    let signup_pending = signup.pending();

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();

        if signup_pending() {
            return;
        }

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
    let input_style = "input input-bordered input-info w-full max-w-md";

    view! { cx,
        <div class="max-w-md mx-auto mt-8">
            <div class="rounded-lg shadow-md p-8">
                <h2 class="text-2xl font-semibold mb-6">"Signup"</h2>
                <form on:submit=on_submit>
                    <div class="mb-4">
                        <label for="username" class=label_style>
                            <span class="label-text">"Username"</span>
                        </label>
                        <input type="username" node_ref=username class=input_style required />
                    </div>
                    <div class="mb-6">
                        <label for="email" class=label_style>
                            <span class="label-text">"Email"</span>
                        </label>
                        <input type="email" node_ref=email class=input_style required />
                    </div>
                    <div class="mb-6">
                        <label for="password" class=label_style>
                            <span class="label-text">"Password"</span>
                        </label>
                        <input type="password" node_ref=password class=input_style required />
                    </div>
                    <div>
                        <button type="submit" class="btn btn-primary py-2 px-4 rounded-md w-full">
                            {move || if signup_pending() {
                                "Loading..."
                            } else {
                                "Signup"
                            }}
                        </button>
                    </div>
                </form>
                <div class="mt-4 text-center">
                    <span>"Already have an account? "</span>
                    <a href="/login" class="link link-info">
                        "Login"
                    </a>
                </div>
            </div>
        </div>

        <ErrorList error_title="Signup failed!".to_string()>
            {move || {
                signup_result().map(|r| r.map(|_| {
                    let goto = use_navigate(cx);
                    // FIXME: error handling
                    goto("/login", NavigateOptions::default()).unwrap()
                }))
            }}
        </ErrorList>
    }
}
