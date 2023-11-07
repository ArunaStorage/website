use crate::utils::structs::WhoamiResponse;
use cfg_if::cfg_if;
use gloo_events::EventListener;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use regex::Regex;
#[allow(unused_imports)]
use std::time::Duration;

#[server(RegisterUser)]
pub async fn register_user(
    displayname: String,
    email: String,
    project: String,
) -> Result<(), ServerFnError> {
    use crate::utils::aruna_api_handlers::ConnectionHandler;

    use crate::utils::login_helpers::{extract_token, LoginResult};

    let LoginResult::ValidToken(token) = extract_token().await else {
        return Err(ServerFnError::ServerError("NotLoggedIn".to_string()));
    };

    ConnectionHandler::aruna_register_user(&token, &displayname, &email, &project)
        .await
        .map_err(|e| ServerFnError::Request(format!("Invalid request: {}", e.to_string())))?;
    Ok(())
}

/// Renders the home page of your application.
#[component]
pub fn RegisterPage() -> impl IntoView {
    // Email regex
    let _regex = Regex::new(
        r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})",
    )
    .unwrap();
    provide_meta_context();

    let register_user = create_server_action::<RegisterUser>();
    let _get_user = use_context::<Resource<(), WhoamiResponse>>().expect("user_state not set");

    let nav = use_navigate();
    let modal_ref = create_node_ref::<html::Div>();
    modal_ref.on_load(move |loaded| {
        let _ = loaded.on_mount(move |mounted| {
            cfg_if! {
                if #[cfg(feature = "hydrate")] {
                    use crate::utils::modal::show_modal;
                    show_modal("registerModal");
                }
            };
            let _show_modal = EventListener::new(&mounted, "hide.bs.modal", move |_event| {
                nav("/", Default::default());
            });
        });
    });
    view! {
        <ActionForm
            on:submit=move |ev| {
                cfg_if! {
                    if #[cfg(feature = "hydrate")] {
                        use crate::utils::modal::hide_modal;
                        hide_modal("registerModal");
                        let _ = window().location().set_href("/activate");
                    }
                };
                let _data = RegisterUser::from_event(&ev).expect("to parse form data");
            }

            action=register_user
        >
            <div class="modal mt-5 fade" id="registerModal" _ref=modal_ref tabindex="-1">
                <div class="modal-dialog modal-sm" role="document">
                    <div class="modal-content">
                        <div class="modal-status bg-info"></div>

                        <div class="modal-body">

                            <div class="text-center py-4">
                                <svg
                                    xmlns="http://www.w3.org/2000/svg"
                                    class="icon icon-tabler icon-tabler-api-app mb-2 text-blue icon-lg"
                                    width="40"
                                    height="40"
                                    viewBox="0 0 24 24"
                                    stroke-width="1"
                                    stroke="currentColor"
                                    fill="none"
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                >
                                    <path stroke="none" d="M0 0h24v24H0z" fill="none"></path>
                                    <path d="M12 15h-6.5a2.5 2.5 0 1 1 0 -5h.5"></path>
                                    <path d="M15 12v6.5a2.5 2.5 0 1 1 -5 0v-.5"></path>
                                    <path d="M12 9h6.5a2.5 2.5 0 1 1 0 5h-.5"></path>
                                    <path d="M9 12v-6.5a2.5 2.5 0 0 1 5 0v.5"></path>
                                </svg>
                                <h3>"Registration required!"</h3>
                                <div class="text-muted">
                                    <div>"Your account is not yet registered, you must register first before you can proceed!"</div>
                                    <div>"Registering indicates your acceptance to our:"</div>
                                    <a href="/tos" class="px-2">
                                        "Terms of Service"
                                    </a>
                                </div>
                            </div>
                            <div class="mx-auto text-left import-left">
                                <div class="mb-3">
                                    <label class="form-label text-left">"Displayname"</label>
                                    <input
                                        type="text"
                                        class="form-control flex-fill"
                                        name="displayname"
                                        placeholder=""
                                    />
                                </div>
                                <div class="mb-3">
                                    <label class="form-label text-left import-left">
                                        "Email"
                                    </label>
                                    <input
                                        type="text"
                                        class="form-control flex-fill"
                                        name="email"
                                        placeholder=""
                                    />
                                </div>
                                <div class="mb-3">
                                    <label class="form-label text-left import-left">
                                        "Project (optional)"
                                    </label>
                                    <input
                                        type="text"
                                        class="form-control flex-fill"
                                        name="project"
                                        placeholder=""
                                    />
                                </div>
                            </div>
                        </div>

                        <div class="modal-footer">
                            <a
                                href="/"
                                class="btn"
                                data-bs-dismiss="modal"
                                data-bs-target="#registerModal"
                            >
                                "Cancel"
                            </a>
                            <button type="submit" class="btn btn-primary ms-auto">
                                <svg
                                    xmlns="http://www.w3.org/2000/svg"
                                    class="icon icon-tabler icon-tabler-plus"
                                    width="24"
                                    height="24"
                                    viewBox="0 0 24 24"
                                    stroke-width="1"
                                    stroke="currentColor"
                                    fill="none"
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                >
                                    <path stroke="none" d="M0 0h24v24H0z" fill="none"></path>
                                    <path d="M12 5l0 14"></path>
                                    <path d="M5 12l14 0"></path>
                                </svg>
                                "Register"
                            </button>
                        </div>
                    </div>
                </div>
            </div>
        </ActionForm>
    }
}

/// Renders the home page of your application.
#[component]
pub fn ActivatePage() -> impl IntoView {
    provide_meta_context();

    let nav = use_navigate();
    let activate_ref = create_node_ref::<html::Div>();
    let get_user = use_context::<Resource<(), WhoamiResponse>>().expect("user_state not set");

    activate_ref.on_load(move |loaded| {
        let _ = loaded.on_mount(move |mounted| {
            cfg_if! {
                if #[cfg(feature = "hydrate")] {
                    use crate::utils::modal::show_modal;
                    show_modal("activateModal");
            }};
            let show_modal = EventListener::new(&mounted, "hide.bs.modal", move |_event| {
                nav("/", Default::default());
            });

            on_cleanup(move || {
                get_user.refetch();
                drop(show_modal)
            });
        });
    });

    on_cleanup(move || {
        get_user.refetch();
    });

    view! {
        <div
            class="modal fade"
            id="activateModal"
            tabindex="-1"
            _ref=activate_ref
            style="display: block; padding-left: 0px;"
        >
            <div class="modal-dialog modal-sm modal-dialog-centered" role="document">
                <div class="modal-content">
                    <button
                        type="button"
                        class="btn-close"
                        data-bs-dismiss="modal"
                        aria-label="Close"
                        data-bs-target="#activateModal"
                    ></button>
                    <div class="modal-status bg-success"></div>
                    <div class="modal-body text-center py-4">
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            class="icon icon-tabler icon-tabler-user-check icon-lg text-green"
                            width="40"
                            height="40"
                            viewBox="0 0 24 24"
                            stroke-width="1"
                            stroke="currentColor"
                            fill="none"
                            stroke-linecap="round"
                            stroke-linejoin="round"
                        >
                            <path stroke="none" d="M0 0h24v24H0z" fill="none"></path>
                            <path d="M8 7a4 4 0 1 0 8 0a4 4 0 0 0 -8 0"></path>
                            <path d="M6 21v-2a4 4 0 0 1 4 -4h4"></path>
                            <path d="M15 19l2 2l4 -4"></path>
                        </svg>
                        <h3 class="pt-4">"Registration complete!"</h3>
                        <div class="text-muted">
                            "Your account will be reviewed and activated by an administrator. We will send you a notification on activation."
                        </div>
                    </div>
                    <div class="modal-footer">
                        <div class="w-100">
                            <div class="row">
                                <div class="col">
                                    <a
                                        class="btn w-100"
                                        data-bs-dismiss="modal"
                                        data-bs-target="#activateModal"
                                    >
                                        "Close"
                                    </a>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
