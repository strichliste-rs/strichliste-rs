use leptos::{prelude::*, task::spawn_local};
use thaw::{
    Button, ButtonAppearance, ButtonSize, Field, FieldOrientation, Flex, FlexAlign, Grid, GridItem,
    Input,
};

use crate::{
    backend::core::behaviour::split_cost::split_cost,
    frontend::{
        component::{
            multi_user_selection::SelectMultiUser, single_user_selection::SelectSingleUser,
        },
        shared::throw_error_none_view,
    },
};

#[component]
pub fn Show() -> impl IntoView {
    let primary_user = RwSignal::new(String::new());
    let secondary_users = RwSignal::new(Vec::<String>::new());
    let money_input = RwSignal::new(String::new());
    let description_input = RwSignal::new(String::new());

    let error_signal = RwSignal::new(String::new());

    // don't see how I can pass a Vec<String> to a server function with ActionForms
    let on_click = move |_| {
        spawn_local(async move {
            if let Err(e) = split_cost(
                primary_user.get_untracked(),
                Some(secondary_users.get_untracked()),
                money_input.get_untracked(),
                description_input.get_untracked(),
            )
            .await
            {
                error_signal.update(|value| *value = e.to_string());
            }
        })
    };
    view! {
        {move || {
            let msg = error_signal.get();
            match msg.len() {
                0 => ().into_any(),
                _ => throw_error_none_view(msg),
            }
        }}
        <Flex vertical=true align=FlexAlign::Center>
            <Grid cols=2 x_gap=10 y_gap=10>
                <GridItem>
                    <div class="pt-4">
                        <SelectSingleUser title=String::from("Who are you?") input=primary_user />
                    </div>
                </GridItem>
                <GridItem>
                    <div class="pt-4">
                        <SelectMultiUser
                            title=String::from("Who do you want to split the cost with?")
                            users_input=secondary_users
                        />
                    </div>
                </GridItem>
                <GridItem>
                    <Field label="How much?" orientation=FieldOrientation::Horizontal required=true>
                        <Input value=money_input />
                    </Field>
                    <Field label="Description" orientation=FieldOrientation::Horizontal>
                        <Input value=description_input />
                    </Field>
                </GridItem>
                <GridItem>
                    <Button
                        class="w-full"
                        appearance=ButtonAppearance::Primary
                        size=ButtonSize::Large
                        on_click=on_click
                    >
                        "Split cost"
                    </Button>
                </GridItem>
            </Grid>
        </Flex>
    }
}
