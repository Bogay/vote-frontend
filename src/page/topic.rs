use crate::api::{
    get_comments, get_my_vote, get_one_topic, CreateOptionInput, CreateTopic, CreateTopicInput,
    CreateVote, CreateVoteInput, GetCommentsInput,
};
use crate::component::*;
use crate::state::GlobalState;
use leptos::ev::SubmitEvent;
use leptos::*;
use leptos_router::*;

#[component]
pub fn TopicPage(cx: Scope) -> impl IntoView {
    let state = expect_context::<RwSignal<GlobalState>>(cx);
    let is_login = move || state().token().is_some();
    let params = use_params_map(cx);
    let id = params.with(|params| params.get("id").unwrap().to_string());
    let (id, _) = create_signal(cx, id);
    let topic = create_local_resource(cx, id, move |id| async move { get_one_topic(id).await });
    let create_vote = create_server_action::<CreateVote>(cx);
    let create_vote_pending = create_vote.pending();
    let create_vote_result = create_vote.value();
    let my_vote = create_local_resource(
        cx,
        move || {
            (
                state().token().map(|t| t.to_string()),
                topic.read(cx).and_then(|t| {
                    // FIXME: error handling
                    t.ok()
                }),
            )
        },
        move |(token, topic)| async move {
            let (Some(token), Some(topic)) = (token, topic) else {
                    return None;
                };

            Some(
                get_my_vote(
                    token.to_string(),
                    crate::api::GetMyVoteInput { topic_id: topic.id },
                )
                .await,
            )
        },
    );
    let comments = create_resource(cx, id, move |id| async move {
        get_comments(GetCommentsInput { topic_id: id }).await
    });
    let comments_view = move || {
        comments.read(cx).map(|comments| {
            comments.map(|comments| {
                if comments.is_empty() {
                    view! { cx,
                        <p class="p-8 w-full text-center">
                            "No comments"
                        </p>
                    }
                    .into_view(cx)
                } else {
                    comments
                        .into_iter()
                        .map(|comment| {
                            let (comment, _) = create_signal(cx, comment);
                            view! { cx, <CommentCard comment=comment /> }
                        })
                        .collect_view(cx)
                }
            })
        })
    };

    let refetch = move || {
        topic.refetch();
        my_vote.refetch();
    };

    create_effect(cx, move |_| {
        let r = create_vote_result();
        if r.is_some() {
            refetch();
        }
        r
    });

    view! { cx,
        <Transition fallback=move || view! { cx, <p>"Loading..." <span class="loading loading-spinner"></span></p> }>
            <ErrorList error_title="Topic Page".to_string()>
                {move || topic.read(cx).map(move |topic| {
                    topic.map(|topic| {
                        let (topic, _) = create_signal(cx, topic);
                        let topic_card = {
                            view! { cx,
                                <TopicCard topic=topic show_action=false />
                            }.into_view(cx)
                        };
                        // FIXME: error handling
                        let my_vote = my_vote.read(cx).and_then(|v| v).and_then(|v| v.ok());
                        let option_cards = topic().options.iter().map(|opt| {
                            let (opt, _) = create_signal(cx, opt.clone());
                            let vote = move |_| {
                                if create_vote_pending() {
                                    return;
                                }
                                let topic_id = topic().id;
                                let input = CreateVoteInput {
                                    topic_id: topic_id.clone(),
                                    option_id: opt().id,
                                };
                                create_vote.dispatch(CreateVote {
                                    token: state().token().expect("should not called if not logged in").to_string(),
                                    input,
                                });
                            };
                            let extra_class = my_vote.as_ref().and_then(|v| (opt().id == v.option_id).then(|| "bg-primary")).unwrap_or_default();
                            view! { cx,
                                <OptionCard
                                    option=opt
                                    extra_class=extra_class.to_string()
                                    action=Some(move || view! { cx,
                                        <button
                                            class="btn"
                                            on:click=vote
                                            class:btn-disabled=move ||create_vote_pending() || !is_login()
                                            class:btn-info=move || !create_vote_pending() && is_login()
                                        >
                                            {move || if create_vote_pending() {
                                                "Loading..."
                                            } else {
                                                "+1"
                                            }}
                                        </button>
                                    })
                                />
                            }
                        }).collect_view(cx);

                        view! { cx,
                            <div class="p-4 md:p-16 w-full mx-auto grid grid-cols-1 lg:grid-cols-2">
                                <div class="flex flex-col item-center">
                                    {topic_card}
                                    {move || (!is_login()).then(|| {
                                        view! { cx,
                                            <h2 class="text-center">
                                                "Login to vote."
                                            </h2>
                                        }
                                    })}
                                    {option_cards}
                                </div>
                                <div class="flex flex-col">
                                    <h2 class="text-3xl pb-4">"Comments"</h2>
                                    {comments_view}
                                    {move || view!{ cx, <CreateCommentCard
                                        id=id()
                                        // FIXME: reloading comments
                                        // comments.refetch()
                                        after_submit=|_| {}
                                    />}}
                                </div>
                            </div>
                            // {create_vote_result().map(|r| r.map(|_| {
                            //     refetch();
                            // }))}
                        }
                    })
                })}
            </ErrorList>
        </Transition>
    }
}

#[component]
pub fn CreateTopicPage(cx: Scope) -> impl IntoView {
    use leptos::html::Input;

    let description: NodeRef<Input> = create_node_ref(cx);
    let starts_at: NodeRef<Input> = create_node_ref(cx);
    let ends_at: NodeRef<Input> = create_node_ref(cx);

    // let init_options = vec![(0, create_signal(cx, CreateOptionInput::default()))];
    let init_options = vec![];
    let (options, set_options) = create_signal(cx, init_options);
    let mut next_id = 1;
    let add_option = move |_| {
        set_options.update(move |options| {
            options.push((next_id, create_signal(cx, CreateOptionInput::default())));
        });
        next_id += 1;
    };

    let create_topic = create_server_action::<CreateTopic>(cx);
    let create_topic_result = create_topic.value();
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

    let input_style = "input input-bordered input-info w-full max-w-md";

    view! { cx,
        <div class="max-w-md mx-auto mt-8">
            <div class="rounded-lg shadow-md p-8">
                <h2 class="text-2xl font-semibold mb-6">"Create Topic"</h2>
                <form on:submit=on_submit>
                    <div class="mb-4">
                        <label for="description" class="">
                            <span class="label-text">"Description"</span>
                        </label>
                        <input type="text" id="description" name="description" node_ref=description class=input_style required />
                    </div>
                    <div class="mb-4">
                        <label for="starts_at" class="">
                            <span class="label-text">"Starts At"</span>
                        </label>
                        <input type="datetime-local" id="starts_at" name="starts_at" node_ref=starts_at class=input_style required />
                    </div>
                    <div class="mb-4">
                        <label for="ends_at" class="">
                            <span class="label-text">"Ends At"</span>
                        </label>
                        <input type="datetime-local" id="ends_at" name="ends_at" node_ref=ends_at class=input_style required />
                    </div>
                    <div>
                        <h3 class="text-lg font-semibold mb-2">"Options"</h3>
                        <div id="options-container">
                            <For
                                each=options
                                key=|option| option.0
                                view=move |cx, (_, (option, set_option))| {
                                    // seems to be a bug, this value is actually used
                                    #[allow(unused)]
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
                                                class="textarea textarea-info mt-2 w-full max-w-md"
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
                            class="btn btn-info py-2 px-4"
                            on:click=add_option
                        >
                            "Add Option"
                        </button>
                    </div>
                    <div class="mt-6">
                        <button
                            type="submit"
                            class="btn btn-success py-2 px-4 w-full"
                        >
                            "Create"
                        </button>
                    </div>
                </form>
            </div>
        </div>

        <ErrorList error_title="Create Topic Failed".to_string()>
            {move || create_topic_result().map(|resp| resp.map(|_| {
                let goto = use_navigate(cx);
                // FIXME: error handling
                goto("/", NavigateOptions::default())
            }))}
        </ErrorList>
    }
}
