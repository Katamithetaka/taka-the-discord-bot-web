use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);
    
    view! { cx,
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>
        
        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="/logs" view=Logs/>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(cx, 0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! { cx,
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}



#[server(GetLogs, "/api")]
pub async fn get_logs() -> Result<Vec<String>, ServerFnError> {
    let path = std::env::var("LOG_FILE_DIRECTORY");
    let last_modified_file = std::fs::read_dir(path.unwrap_or(String::from("./logs")))
    .expect("Couldn't access local directory")
    .flatten() // Remove failed
    .filter(|f| f.metadata().unwrap().is_file()) // Filter out directories (only consider files)
    .max_by_key(|x| x.metadata().unwrap().modified().unwrap()).unwrap(); // Get the most recently modified file

    let value = std::fs::read_to_string(format!("{}", last_modified_file.path().to_str().unwrap())).unwrap();
    let return_val = value.lines().map(|c| c.to_string()).collect();
    Ok(return_val)
}

// #[server(FetchCats, "/api")]
// pub async fn fetch_cats(how_many: u32) -> Result<Vec<String>, ServerFnError> {
//     // pretend we're fetching cat pics
//     Ok(vec![how_many.to_string()])
//   }

// #[component]
fn Logs(cx: Scope) -> impl IntoView {
    let (cat_count, _set_cat_count) = create_signal::<u32>(cx, 1);
    let (_pending, set_pending) = create_signal(cx, false);
    let cats =
        create_resource(cx, move || cat_count.get(), |count| get_logs());
    view! { cx,
      <div>
      <link rel="stylesheet" href="https://unpkg.com/terminal.css@0.7.2/dist/terminal.min.css" />
      <link rel="stylesheet" href="https://unpkg.com/terminal.css@0.7.1/dist/terminal.min.css" />
      <link rel="stylesheet" href="https://unpkg.com/terminal.css@0.7.1/dist/terminal.min.css" />
        <Transition
          fallback=move || view! { cx, <p>"Loading..."</p>}
          set_pending=set_pending.into()
        >
          {move || {
              cats.read(cx).map(|data| match data {
                Err(_) => view! { cx,  <pre>"Error"</pre> }.into_view(cx),
                Ok(cats) => {
                    view! {
                        cx,
                        <p>
                        {cats.iter().map(|cat| {
                            view! { cx, 
                                <p> {cat}<br/> </p>
                            }
                        }).collect_view(cx)}
                        </p>
                    }.into_view(cx)

                }
            })
          }}
        </Transition>
      </div>
    }
}


/// 404 - Not Found
#[component]
fn NotFound(cx: Scope) -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>(cx);
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! { cx,
        <h1>"Not Found"</h1>
    }
}
