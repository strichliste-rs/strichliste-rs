use leptos::{
    html::{self, Audio},
    leptos_dom::logging::console_log,
    prelude::*,
    task::spawn_local,
};
use leptos_router::hooks::use_params_map;
use tracing::error;

use crate::models::{Article, ArticleDB, Barcode, BarcodeDB};

#[server]
pub async fn get_article(article_id: i64) -> Result<Article, ServerFnError> {
    use crate::backend::ServerState;
    let state: ServerState = expect_context();
    use axum::http::StatusCode;
    use leptos_axum::ResponseOptions;

    let response_opts: ResponseOptions = expect_context();

    let article = Article::get_from_db(&*state.db.lock().await, article_id).await;

    let article = match article {
        Ok(value) => value,
        Err(e) => {
            response_opts.set_status(StatusCode::INTERNAL_SERVER_ERROR);
            return Err(ServerFnError::new(format!(
                "Error getting article from db: {}",
                e
            )));
        }
    };

    match article {
        None => {
            response_opts.set_status(StatusCode::BAD_REQUEST);
            Err(ServerFnError::new(format!(
                "Unknown Article id '{}'",
                article_id
            )))
        }

        Some(value) => Ok(value),
    }
}

#[server]
pub async fn update_article(id: i64, name: String, cost: String, barcodes: Option<Vec<Barcode>>) -> Result<(), ServerFnError> {
    use crate::backend::ServerState;
    let state: ServerState = expect_context();
    use axum::http::StatusCode;
    use leptos_axum::ResponseOptions;
    use leptos_axum::redirect;

    let response_opts: ResponseOptions = expect_context();

    let mut article = get_article(id).await?;

    article.name = name;
    article.cost = match cost.clone().try_into() {
      Ok(value) => value,
      Err(e) => {
          response_opts.set_status(StatusCode::BAD_REQUEST);
          return Err(ServerFnError::new(&format!("Failed to convert '{}' to internal money representation: {}", cost, e)))
      }
    };

    let article_db: ArticleDB = (&article).into();

    match article_db.update(&*state.db.lock().await).await {
        Ok(_) => {},
        Err(e) => {
          response_opts.set_status(StatusCode::INTERNAL_SERVER_ERROR);
          error!("Failed to update article_db: {}", e);
          return Err(ServerFnError::new(format!("Failed to update article: {}", e)));
        },
    }

    match barcodes {
        None => {},
        Some(barcodes) => {
            for barcode in barcodes.iter() {
                let checking_barcode = &barcode.0;
                let barcode_used = BarcodeDB::get_article_id_from_barcode(&*state.db.lock().await, &checking_barcode).await;

                match barcode_used {
                    Ok(value) => {
                        match value {
                            None => {},
                            Some(barcode_article_id) => {
                                if barcode_article_id != id {
                                    let barcode_taken_article = get_article(barcode_article_id).await?;
                                    response_opts.set_status(StatusCode::BAD_REQUEST);
                                    return Err(ServerFnError::new(&format!("The barcode '{}' is already used by article '{}'!", checking_barcode, barcode_taken_article.name)))
                                }
                            },
                        }
                    },
                    Err(e) => {
                        response_opts.set_status(StatusCode::INTERNAL_SERVER_ERROR);
                        error!("Failed to check if barcode is used: {}", e);
                        return Err(ServerFnError::new(&format!("Failed to check if barcode '{}' is already used: {}", barcode.0, e)))
                    }
                }
            }

            let result = article_db.set_barcodes(&*state.db.lock().await, barcodes).await;

            match result {
                Err(e) => {
                    response_opts.set_status(StatusCode::INTERNAL_SERVER_ERROR);
                    error!("Failed to set barcodes: {}", e);
                    return Err(ServerFnError::new(&format!("Failed to set barcodes: {}", e)));
                },

                Ok(_) => {},
            }
        },
    }


    redirect("/articles");

    Ok(())
}

#[component]
pub fn Edit() -> impl IntoView {
    let params = use_params_map();
    let article_id_string = params
        .read_untracked()
        .get("article_id")
        .unwrap_or_default();

    let article_id = article_id_string.parse::<i64>();
    let article_id = match article_id {
        Ok(value) => value,
        Err(err) => {
            return view! {
                <p class="text-red-400 text-center">"Failed to get id from params map: "{err.to_string()}</p>
            }
            .into_any();
        }
    };

    let article_resource = OnceResource::new(get_article(article_id));

    view! {
        {move || match article_resource.get() {
            None => {
                return view! {
                    <p class="text-white text-center">"Loading article..."</p>
                }.into_any();
            },

            Some(value) => {
                let article = match value {
                  Ok(value) => value,
                  Err(e) => {
                      let error_msg = match e {
                          ServerFnError::ServerError(msg) => msg,
                          _ => e.to_string(),
                      };
                      return view!{
                          <p class="text-red-400 text-center">"Failed to load Article: "{error_msg}</p>
                      }.into_any();
                  }
                };

                return view!{
                    <SingleArticleView article/>
                }.into_any()
            }
        }}
    }
    .into_any()
}

#[component]
fn SingleArticleView(article: Article) -> impl IntoView {
    let name_node = NodeRef::<html::Input>::new();
    let cost_node = NodeRef::<html::Input>::new();

    let new_barcode_node = NodeRef::<html::Input>::new();

    let barcodes_signal = RwSignal::new(article.barcodes.clone());

    let error_signal = RwSignal::new(String::new());

    let class_css = "flex flex-col gap-5";
    let input_css = "ml-5 text-black rounded-[5px] text-center";

    let clone = article.clone();

    let on_click = move |_| {
        let mut article = clone.clone();
        article.name = name_node.get().unwrap().value();
        // console_log("Hello");

        spawn_local(async move {
            let Article { id, name: _, cost: _, sounds: _, barcodes: _ } = article;

            let name = name_node.get_untracked().expect("name input should be mounted").value();
            let cost = cost_node.get_untracked().expect("name input should be mounted").value();

            let barcodes =barcodes_signal.get_untracked();
            match update_article(id.unwrap(), name, cost, Some(barcodes)).await {
                Err(e) => {
                    let msg = match e {
                        ServerFnError::ServerError(msg) => msg,
                        _ => e.to_string(),
                    };

                    error_signal.set(msg);
                },

                Ok(_) => {},
            }
        });
    };
    return view! {
        {
            move || {
                let msg = error_signal.get();

                match msg.len() {
                    0 => view! {}.into_any(),
                    _ => view! {
                        <div class="bg-red-400 p-5">
                            <p class="text-white text-center">"Failed to update article: "{msg}</p>
                        </div>
                    }.into_any(),
                }
            }
        }
        <div class="flex flex-col items-center pt-5 gap-10 text-[1.25em]">
            <div class="flex justify-center pt-5">
                <div class=format!("{} items-end", {class_css})>
                    <a class="text-white">"Name:"</a>
                    <a class="text-white">"Cost:"</a>

                </div>
                <div class=format!("{} items-center", {class_css})>
                    <input class={input_css} type="text" value={article.name} node_ref=name_node/>
                    <input class={input_css} type="text" value={article.cost.format()} node_ref=cost_node/>
                </div>
            </div>
            <div>
                <table class="w-full text-white">
                    <tr class="bg-black">
                        <th>"Barcodes"</th>
                    </tr>
                { move || 
                    barcodes_signal.get().iter().map(|barcode| {
                        let code = barcode.clone().0;
                        view! {
                            <tr class="even:bg-gray-700 odd:bg-gray-500">
                                <td>
                                    <p>{code}</p>
                                </td>
                            </tr>
                        }
                    }).collect_view()
                }
                </table>
            </div>
            <div class="flex justify-center pt-5 gap-5">
                <input class="text-black rounded-[5px] text-center" node_ref=new_barcode_node/>
                <div class="w-[10vw]">
                    <button type="button" class="w-full bg-indigo-700 hover:bg-pink-700 text-white font-bold py-2 px-4 mb-6 rounded"
                        on:click=move |_| {
                            let node = new_barcode_node.get().expect("new_barcode_input should be mounted!");
                            let new_barcode = node.value();
                            if new_barcode.len() == 0 {
                                return;
                            }
                            // console_log(&format!("Pushing value: {}", new_barcode));
                            barcodes_signal.write().push(Barcode(new_barcode));
                            // console_log(&format!("{:#?}", barcodes_signal.get()));
                            node.set_value("");
                        }
                    >"Add Barcode"</button>
                </div>
            </div>

            // Last element
            <div class="w-[30vw]">
                <input class="w-full bg-indigo-700 hover:bg-pink-700 text-white font-bold py-2 px-4 mb-6 rounded" type="submit" value="Upate article" on:click=on_click/>
            </div>
        </div>
    }
    .into_any();
}
