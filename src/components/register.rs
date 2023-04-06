use cfg_if::cfg_if;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[server(RegisterUser, "/web")]
pub async fn register_user(
    #[allow(unused_variables)] cx: Scope,
    displayname: String,
    email: String,
    project: String,
) -> Result<(), ServerFnError> {
    dbg!(displayname.to_string(), email, project);

    // Test if registered -> SetCookie
    cfg_if! {
        if #[cfg(feature = "ssr")] {
            use http::{header::SET_COOKIE, HeaderValue};
            use leptos_actix::ResponseOptions;
            use actix_web::cookie::{Cookie, SameSite};
            use actix_web::cookie::time::{Duration, OffsetDateTime};
            let response = use_context::<ResponseOptions>(cx);

            let mut test_cookie = Cookie::build("registered", "true").path("/").finish();
            test_cookie.set_same_site(SameSite::None);

            let expiry = OffsetDateTime::now_utc() + Duration::seconds(60 * 60 * 24 * 7);
            test_cookie.set_expires(expiry);

            if let Some(res_options) = response {
                res_options.append_header(SET_COOKIE, HeaderValue::from_str(test_cookie.to_string().as_ref()).unwrap())
            }
        }
    }

    Ok(())
}

/// Renders the home page of your application.
#[component]
pub fn RegisterPage(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);

    let (registered, set_registered) = create_signal(cx, None::<String>);
    cfg_if! {
        if #[cfg(feature = "hydrate")] {
            use wasm_bindgen::JsCast;
            use crate::utils::modal::toggle_modal;
            use crate::utils::parse_cookies::parse_cookies;
            toggle_modal("registerModal");
            let doc = document().unchecked_into::<web_sys::HtmlDocument>();
            set_registered(parse_cookies(doc.cookie().unwrap_or_default(), "registered"));
        }
    };

    let register_user = create_server_multi_action::<RegisterUser>(cx);

    let register_form = view! {cx,

        <MultiActionForm on:submit=move |ev| {
            let data = RegisterUser::from_event(&ev).expect("to parse form data");
            // silly example of validation: if the todo is "nope!", nope it
            if data.displayname == "nope!" {
                // ev.prevent_default() will prevent form submission
                ev.prevent_default();
            }
        }
        action=register_user
    >
    <div class="modal mt-5" id="registerModal" tabindex="-1">
        <div class="modal-dialog modal-sm" role="document">
            <div class="modal-content">
                <div class="modal-status bg-info"></div>


                <div class="modal-body">

                    <div class="text-center py-4">
                        <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-api-app mb-2 text-blue icon-lg" width="40" height="40" viewBox="0 0 24 24" stroke-width="1" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round">
                            <path stroke="none" d="M0 0h24v24H0z" fill="none"></path>
                            <path d="M12 15h-6.5a2.5 2.5 0 1 1 0 -5h.5"></path>
                            <path d="M15 12v6.5a2.5 2.5 0 1 1 -5 0v-.5"></path>
                            <path d="M12 9h6.5a2.5 2.5 0 1 1 0 5h-.5"></path>
                            <path d="M9 12v-6.5a2.5 2.5 0 0 1 5 0v.5"></path>
                        </svg>
                        <h3>"Registration required!"</h3>
                        <div class="text-muted">"Your account is not yet registered, please register first before you can proceed!"</div>
                    </div>
                    <div class="mx-auto">
                        <div class="mb-3">
                            <label class="form-label text-left">"Displayname"</label>
                            <input type="text" class="form-control flex-fill" name="displayname" placeholder="" />
                        </div>
                        <div class="mb-3">
                            <label class="form-label text-left">"Email (optional)"</label>
                            <input type="text" class="form-control flex-fill" name="email" placeholder="" />
                        </div>
                        <div class="mb-3">
                            <label class="form-label text-left">"Project (optional)"</label>
                            <input type="text" class="form-control flex-fill" name="project" placeholder="" />
                        </div>
                    </div>
                </div>

                <div class="modal-footer">
                    <a href="#" class="btn" data-bs-dismiss="modal">
                    "Cancel"
                    </a>
                    <button type="submit" class="btn btn-primary ms-auto" data-bs-dismiss="modal">
                    <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-plus" width="24" height="24" viewBox="0 0 24 24" stroke-width="1" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round">
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
    </MultiActionForm>
    };

    view! {cx,
        {register_form}
    }
}
