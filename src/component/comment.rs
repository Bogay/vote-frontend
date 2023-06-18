use crate::api::{Comment, CreateComment, CreateCommentInput};
use crate::state::GlobalState;
use leptos::ev::MouseEvent;
use leptos::*;

#[component]
pub fn CommentCard(cx: Scope, #[prop(into)] comment: Signal<Comment>) -> impl IntoView {
    view! { cx,
        <div class="card">
            <div class="card-body">
                <p>{comment().content}</p>
            </div>
        </div>
    }
}

#[component]
pub fn CreateCommentCard<F>(cx: Scope, id: String, after_submit: F) -> impl IntoView
where
    F: Fn(MouseEvent) + 'static,
{
    let (id, _) = create_signal(cx, id);
    let (content, set_content) = create_signal(cx, "".to_string());
    let create_comment = create_server_action::<CreateComment>(cx);
    let create_comment_pending = create_comment.pending();
    let state = expect_context::<RwSignal<GlobalState>>(cx);
    let is_login = move || state().token().is_some();

    let on_submit = move |ev| {
        if !is_login() {
            return;
        }

        create_comment.dispatch(CreateComment {
            token: state().token().unwrap().to_string(),
            input: CreateCommentInput {
                topic_id: id(),
                content: content(),
            },
        });

        set_content("".to_string());

        after_submit(ev);
    };
    let submit_btn_label = move || {
        if is_login() {
            if create_comment_pending() {
                "Loading..."
            } else {
                "Submit"
            }
        } else {
            "Login to continue"
        }
    };

    view! { cx,
        <div class="card">
            <h2 class="card-title">"Leave your comment here:"</h2>
            <textarea
                class="textarea"
                on:input = move |ev| {
                    set_content(event_target_value(&ev));
                }
                prop:value=content
            />
            <button
                class:btn-disabled=move || !is_login()
                class:btn-primary=is_login
                on:click=on_submit
                class="btn"
            >
                {submit_btn_label}
            </button>
        </div>
    }
}
