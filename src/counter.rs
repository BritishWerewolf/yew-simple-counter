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
            <div class="grid items-center w-full h-full grid-cols-3 gap-4 justify-items-center">
                <div class="w-full h-full col-span-1 text-5xl">
                    <button class="w-full h-full p-4 bg-red-200 rounded-md" onclick={ctx.link().callback(|_| Self::Message::Decrement)}>{ "-1" }</button>
                </div>
                <div class="col-span-1 text-9xl">
                    <p>{ self.value }</p>
                </div>
                <div class="w-full h-full col-span-1 text-5xl">
                    <button class="w-full h-full p-4 bg-green-200 rounded-md" onclick={ctx.link().callback(|_| Self::Message::Increment)}>{ "+1" }</button>
                </div>
                <div class="w-full h-full col-span-3 text-5xl">
                    <button class="w-full h-full p-4 bg-yellow-200 rounded-md" onclick={ctx.link().callback(|_| Self::Message::Reset)}>{ "Reset" }</button>
                </div>
            </div>
        }
    }
}
