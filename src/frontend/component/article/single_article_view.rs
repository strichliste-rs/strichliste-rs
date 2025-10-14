use leptos::{
    html::{self},
    prelude::*,
    task::spawn_local,
};
use thaw::{Flex, FlexAlign, Input, Label};

use crate::{
    backend::core::{behaviour::update_article::update_article, Article, Barcode, BarcodeDiff},
    frontend::shared::throw_error_none_view,
};

#[component]
pub fn SingleArticleView(article: Article) -> impl IntoView {
    let name = RwSignal::new(article.name.clone());
    let cost = RwSignal::new(article.cost.format());

    let new_barcode_node = NodeRef::<html::Input>::new();

    let barcodes_signal = RwSignal::new(article.barcodes.clone());
    let barcodes_diff_signal = RwSignal::new(Vec::<BarcodeDiff>::new());

    let error_signal = RwSignal::new(String::new());

    let class_css = "flex flex-col gap-5";
    let input_css = "ml-5 text-black rounded-[5px] text-center";

    let clone = article.clone();

    let on_click = move |_| {
        let mut article = clone.clone();
        article.name = name.get();

        spawn_local(async move {
            let Article {
                id,
                name: _,
                cost: _,
                sounds: _,
                barcodes: _,
            } = article;

            let barcodes = barcodes_diff_signal.get_untracked();
            if let Err(e) = update_article(
                id,
                name.get_untracked(),
                cost.get_untracked(),
                Some(barcodes),
            )
            .await
            {
                let msg = match e {
                    ServerFnError::ServerError(msg) => msg,
                    _ => e.to_string(),
                };

                error_signal.set(msg);
            }
        });
    };
    view! {
        {move || {
            let msg = error_signal.get();
            match msg.len() {
                0 => ().into_any(),
                _ => throw_error_none_view(msg),
            }
        }}
        <div class="flex flex-col items-center pt-5 gap-10 text-[1.25em]">
            <Flex>
                <Flex
                    vertical=true
                    align=FlexAlign::Center
                    style=format!("{} items-center", class_css)
                >
                    <Label>"Name: "</Label>
                    <Label>"Cost: "</Label>
                </Flex>
                <Flex
                    vertical=true
                    align=FlexAlign::Center
                    style=format!("{} items-center", class_css)
                >
                    <Input class=input_css value=name />
                    <Input class=input_css value=cost />
                </Flex>
            </Flex>
            <div>
                <table class="w-full text-white border-collapse border-spacing-5">
                    <tr class="bg-black">
                        <th class="pl-2">"Barcodes"</th>
                        <th></th>
                    </tr>
                    {move || {
                        barcodes_signal
                            .get()
                            .iter()
                            .map(|barcode| {
                                let code = barcode.clone().0;
                                view! {
                                    <tr class="even:bg-gray-700 odd:bg-gray-500 text-center">
                                        <td class="px-2">
                                            <p>{code.clone()}</p>
                                        </td>
                                        <td class="px-2">
                                            <button
                                                class="size-8 pt-2"
                                                on:click=move |_| {
                                                    barcodes_signal
                                                        .update(|vec| {
                                                            _ = vec
                                                                .remove(
                                                                    vec
                                                                        .iter()
                                                                        .position(|elem| elem.0 == code)
                                                                        .expect("element should be in list!"),
                                                                );
                                                        });
                                                    barcodes_diff_signal
                                                        .write()
                                                        .push(BarcodeDiff::Removed(code.clone()));
                                                }
                                            >
                                                <svg
                                                    viewBox="0 0 32 32"
                                                    version="1.1"
                                                    xmlns="http://www.w3.org/2000/svg"
                                                    xmlns:xlink="http://www.w3.org/1999/xlink"
                                                    xmlns:sketch="http://www.bohemiancoding.com/sketch/ns"
                                                    fill="#ed333b"
                                                    style="--darkreader-inline-fill: var(--darkreader-background-ed333b, #a90f16);"
                                                    data-darkreader-inline-fill=""
                                                >
                                                    <g id="SVGRepo_bgCarrier" stroke-width="0"></g>
                                                    <g
                                                        id="SVGRepo_tracerCarrier"
                                                        stroke-linecap="round"
                                                        stroke-linejoin="round"
                                                    ></g>
                                                    <g id="SVGRepo_iconCarrier">
                                                        <title>cross-circle</title>
                                                        <desc>Created with Sketch Beta.</desc>
                                                        <defs></defs>
                                                        <g
                                                            id="Page-1"
                                                            stroke="none"
                                                            stroke-width="1"
                                                            fill="none"
                                                            fill-rule="evenodd"
                                                            sketch:type="MSPage"
                                                            style="--darkreader-inline-stroke: none;"
                                                            data-darkreader-inline-stroke=""
                                                        >
                                                            <g
                                                                id="Icon-Set"
                                                                sketch:type="MSLayerGroup"
                                                                transform="translate(-568.000000, -1087.000000)"
                                                                fill="#ed333b"
                                                                style="--darkreader-inline-fill: var(--darkreader-background-000000, #ed333b);"
                                                                data-darkreader-inline-fill=""
                                                            >
                                                                <path
                                                                    d="M584,1117 C576.268,1117 570,1110.73 570,1103 C570,1095.27 576.268,1089 584,1089 C591.732,1089 598,1095.27 598,1103 C598,1110.73 591.732,1117 584,1117 L584,1117 Z M584,1087 C575.163,1087 568,1094.16 568,1103 C568,1111.84 575.163,1119 584,1119 C592.837,1119 600,1111.84 600,1103 C600,1094.16 592.837,1087 584,1087 L584,1087 Z M589.717,1097.28 C589.323,1096.89 588.686,1096.89 588.292,1097.28 L583.994,1101.58 L579.758,1097.34 C579.367,1096.95 578.733,1096.95 578.344,1097.34 C577.953,1097.73 577.953,1098.37 578.344,1098.76 L582.58,1102.99 L578.314,1107.26 C577.921,1107.65 577.921,1108.29 578.314,1108.69 C578.708,1109.08 579.346,1109.08 579.74,1108.69 L584.006,1104.42 L588.242,1108.66 C588.633,1109.05 589.267,1109.05 589.657,1108.66 C590.048,1108.27 590.048,1107.63 589.657,1107.24 L585.42,1103.01 L589.717,1098.71 C590.11,1098.31 590.11,1097.68 589.717,1097.28 L589.717,1097.28 Z"
                                                                    id="cross-circle"
                                                                    sketch:type="MSShapeGroup"
                                                                ></path>
                                                            </g>
                                                        </g>
                                                    </g>
                                                </svg>
                                            </button>
                                        </td>
                                    </tr>
                                }
                            })
                            .collect_view()
                    }}
                </table>
            </div>
            <div class="flex justify-center pt-5 gap-5">
                <input class="text-black rounded-[5px] text-center" node_ref=new_barcode_node />
                <div class="w-[10vw]">
                    <button
                        type="button"
                        class="w-full bg-indigo-700 hover:bg-pink-700 text-white font-bold py-2 px-4 mb-6 rounded"
                        on:click=move |_| {
                            let node = new_barcode_node
                                .get()
                                .expect("new_barcode_input should be mounted!");
                            let new_barcode = node.value();
                            if new_barcode.is_empty() {
                                return;
                            }
                            barcodes_signal.write().push(Barcode(new_barcode.clone()));
                            node.set_value("");
                            barcodes_diff_signal.write().push(BarcodeDiff::Added(new_barcode));
                        }
                    >
                        "Add Barcode"
                    </button>
                </div>
            </div>

            <div class="w-[30vw]">
                <input
                    class="w-full bg-indigo-700 hover:bg-pink-700 text-white font-bold py-2 px-4 mb-6 rounded"
                    type="submit"
                    value="Upate article"
                    on:click=on_click
                />
            </div>
        </div>
    }
    .into_any()
}
