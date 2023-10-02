use yew::prelude::*;

pub struct Counter {
    value: isize,
}

#[derive(Properties, PartialEq)]
pub struct CounterProps {
    #[prop_or(0)]
    pub value: isize,
}

#[derive(Clone)]
pub enum CounterMsg {
    Increment,
    Decrement,
    Reset,
}

impl Component for Counter {
    type Message = CounterMsg;
    type Properties = CounterProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            value: ctx.props().value,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Self::Message::Decrement => self.value -= 1,
            Self::Message::Increment => self.value += 1,
            Self::Message::Reset     => self.value = ctx.props().value,
        };
        // Abstracted as all messages will cause the component to update.
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <button onclick={ctx.link().callback(|_| Self::Message::Decrement)}>{ "-1" }</button>
                <p>{ self.value }</p>
                <button onclick={ctx.link().callback(|_| Self::Message::Increment)}>{ "+1" }</button>
                <br />
                <button onclick={ctx.link().callback(|_| Self::Message::Reset)}>{ "Reset" }</button>
            </div>
        }
    }
}
