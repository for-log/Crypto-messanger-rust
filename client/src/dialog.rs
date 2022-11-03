use web_sys::{FocusEvent, HtmlInputElement};
use yew::{Component, Context, html, Html, Callback, Properties, NodeRef};
use crate::message::Message;

pub enum Msg {
    DoCallback
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub me: usize,
    pub id: usize,
    pub messages: Box<Vec<Message>>,
    pub callback: Callback<String>
}

pub struct Dialog {
    pub name: String,
    pub message: NodeRef,
}

impl Dialog {
    pub fn new(name: String) -> Self {
        Self { name, message: NodeRef::default() }
    }
}

impl Component for Dialog {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let props = ctx.props();
        Self::new(
            format!("User#{}", props.id), 
        )
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let props = ctx.props();
        let onsubmit = link.callback(|event: FocusEvent| {
            event.prevent_default();
            Msg::DoCallback
        });
        html! {
            <div class="current-dialog">
                <div class="dialog-head">
                    <div class="avatar"></div>
                    <p class="name">{self.name.clone()}</p>
                </div>
                <div class="dialog-messages">
                    { props.messages.iter().map(|x| x.view(x.from == props.me)).collect::<Html>() }
                </div>
                <div class="input-holder">
                    <form onsubmit={onsubmit}>
                        <input ref={self.message.clone()} name="message" type="text" placeholder="Message" autocomplete="off" required=true />
                        <button type="submit">
                            <svg xmlns="http://www.w3.org/2000/svg" class="bi bi-send" viewBox="0 0 16 16"><path d="M15.854.146a.5.5 0 0 1 .11.54l-5.819 14.547a.75.75 0 0 1-1.329.124l-3.178-4.995L.643 7.184a.75.75 0 0 1 .124-1.33L15.314.037a.5.5 0 0 1 .54.11ZM6.636 10.07l2.761 4.338L14.13 2.576 6.636 10.07Zm6.787-8.201L1.591 6.602l4.339 2.76 7.494-7.493Z"/></svg>
                        </button>
                    </form>
                </div>
            </div>
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::DoCallback => {
                let input = self.message.cast::<HtmlInputElement>()
                    .unwrap();
                let callback = &ctx.props().callback;
                callback.emit(input.value());
                input.set_value("");
                true
            }
        }
    }

}