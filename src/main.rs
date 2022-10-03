use rand::Rng;
use web_sys::HtmlInputElement;
use yew::events::InputEvent;
use yew::prelude::*;
use yew::{function_component, html, Component};

#[derive(Clone, PartialEq, Debug)]
pub enum ItemState {
    NEW,
    COMPLETED,
    DELETED,
}

#[derive(Clone, PartialEq, Debug)]
struct ItemS {
    id: usize,
    value: String,
    state: ItemState,
}

#[derive(Properties, Clone, Debug, PartialEq)]
struct ItemProps {
    state: ItemState,
    value: String,
    delete_item: Callback<()>,
    complete_item: Callback<()>,
}

#[function_component(Item)]
fn item(
    ItemProps {
        value,
        state,
        delete_item,
        complete_item,
    }: &ItemProps,
) -> Html {
    let onclick = {
        let delete_item = delete_item.clone();
        move |_| delete_item.emit(())
    };
    let onclick_completed = {
        let complete_item = complete_item.clone();
        move |_| complete_item.emit(())
    };

    let state_str = format!("{:?}: ", state);

    html! {
        <div>
            <h1>{ state_str } { value }</h1>
            <button onclick={onclick}>{"X"}</ button>
            <button onclick={onclick_completed}>{"DONE"}</ button>
        </div>
    }
}

pub enum Msg {
    AddItem(),
    DeleteItem(usize),
    CompleteItem(usize),
    UpdateValue(String),
}

#[derive(Clone)]
pub struct App {
    items: Vec<ItemS>,
    value: String,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        App {
            items: vec![ItemS {
                id: rand::thread_rng().gen(),
                value: "FIRST ITEM".to_string(),
                state: ItemState::NEW,
            }],
            value: "".to_string(),
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, _: bool) {}

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
        <div>
            <div>
                <input type="text" value={self.value.clone()} oninput={ctx.link().callback(|e: InputEvent| {
                    let input: HtmlInputElement = e.target_unchecked_into();
                    let value = input.value();
                    log::info!("Value: {:?}", value);
                    Msg::UpdateValue(value)
                })}/>
                <button onclick={ctx.link().callback(move |_| Msg::AddItem())}>{"Add"}</button>
            </div>
            <div>
                {
                    for self.items.iter().cloned().map(|item| {
                        html! {
                            <Item
                            state={item.state}
                            value={item.value}
                            delete_item={ctx.link().callback(move |_| Msg::DeleteItem(item.id))}
                            complete_item={ctx.link().callback(move |_| Msg::CompleteItem(item.id))} />
                        }
                    })
                }
            </div>
        </ div>
        }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateValue(value) => {
                log::info!("To update: {:?}", value);
                self.value = value;
                true
            }
            Msg::AddItem() => {
                log::info!("To add: {:?}", self.value);
                if !self.value.is_empty() {
                    self.items.push(ItemS {
                        id: rand::thread_rng().gen(),
                        value: self.value.clone(),
                        state: ItemState::NEW,
                    });
                    self.value = "".to_string();
                    true
                } else {
                    false
                }
            }
            Msg::DeleteItem(id) => {
                let exists = self.items.iter().find(|item| item.id == id).is_some();
                log::info!("To delete: {:?}, {:?}", id, exists);
                if exists {
                    let items = self.items.clone();
                    self.items = items.into_iter().filter(|item| item.id != id).collect();
                    true
                } else {
                    false
                }
            }
            Msg::CompleteItem(id) => {
                let exists = self.items.iter().find(|item| item.id == id).is_some();
                log::info!("To complete: {:?}, {:?}", id, exists);
                if exists {
                    let items = self.items.clone();
                    self.items = items
                        .into_iter()
                        .map(|item| {
                            if item.id == id {
                                ItemS {
                                    state: ItemState::COMPLETED,
                                    id,
                                    value: item.value,
                                }
                            } else {
                                item
                            }
                        })
                        .collect();
                    true
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    fn changed(&mut self, _: &Context<Self>) -> bool {
        true
    }

    fn destroy(&mut self, _: &Context<Self>) {}
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
