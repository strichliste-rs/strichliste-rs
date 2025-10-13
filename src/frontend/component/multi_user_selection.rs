use leptos::prelude::*;
use thaw::{Button, ButtonAppearance};

use crate::frontend::component::{icon::DeleteIcon, single_user_selection::SelectSingleUser};

#[component]
pub fn SelectMultiUser(
    title: String,
    users_input: RwSignal<Vec<String>>,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let new_user_input = RwSignal::new(String::new());
    view! {
        <div>
            <SelectSingleUser title=title input=new_user_input filter_names=users_input>
                <div class="pt-5">
                    <Button
                        appearance=ButtonAppearance::Primary
                        class="w-full"
                        on_click=move |_| {
                            users_input.write().push(new_user_input.get());
                            new_user_input.write().clear();
                        }
                    >
                        "Add User"
                    </Button>
                </div>
                <div class="flex flex-col items-center pt-5 gap-10 text-[1.25em]">
                    <table class="w-full text-white border-collapse border-spacing-5">
                        {move || {
                            let users_input_value = users_input.get();
                            users_input_value
                                .into_iter()
                                .map(|user| {
                                    view! {
                                        <tr class="even:bg-gray-700 odd:bg-gray-500 text-center">
                                            <td class="px-2">
                                                <p>{user.clone()}</p>
                                            </td>
                                            <td class="px-2">
                                                <button
                                                    class="size-8 pt-2"
                                                    on:click=move |_| {
                                                        users_input
                                                            .update(|vec| {
                                                                _ = vec
                                                                    .remove(
                                                                        vec
                                                                            .iter()
                                                                            .position(|elem| *elem == user)
                                                                            .expect("element should be in list!"),
                                                                    );
                                                            });
                                                    }
                                                >
                                                    <DeleteIcon />
                                                </button>
                                            </td>
                                        </tr>
                                    }
                                })
                                .collect_view()
                        }}
                    </table>
                    {children.as_ref().map(|children| children())}
                </div>
            </SelectSingleUser>
        </div>
    }
}
