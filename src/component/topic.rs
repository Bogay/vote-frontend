use crate::api::{Topic, VoteOption};
use leptos::ev::MouseEvent;
use leptos::*;
use leptos_router::*;

#[component]
pub fn TopicCard(
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
pub fn OptionCard<F>(
    cx: Scope,
    #[prop(into)] option: Signal<VoteOption>,
    #[prop(default = None)] action: Option<F>,
) -> impl IntoView
where
    F: FnMut(MouseEvent) -> () + 'static,
{
    let option = option();

    view! { cx,
        <div class="card card-compact w-96 bg-base-100 m-4">
            <div class="card-title">
                {option.label}
            </div>
            <div class="card-body">
                <p>{option.description}</p>
            </div>
            {action.map(|action| {
                view! { cx,
                    <div class="card-actions justify-end">
                        <button
                            class="btn btn-sm btn-square btn-accent"
                            on:click=action
                        >"+"</button>
                    </div>
                }
            })}
        </div>
    }
}
