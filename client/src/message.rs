use yew::{Component, Context, html, Html};

#[derive(PartialEq, Clone, Debug)]
pub struct Message {
    pub id: usize,
    pub from: usize,
    pub content: String,
}

impl Message {
    pub fn new(id: usize, from: usize, content: String) -> Self {
        Self { id, from, content }
    }
    pub fn view(&self, is_me: bool) -> Html {
        html! {
            <div class={ if !is_me {format!("message mid{}", self.id)} else {format!("message me mid{}", self.id)}}>
              <div class="avatar"></div>
              <p class="content">{self.content.clone()}</p>
            </div>
        }
    }
}

impl Component for Message {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self::new(
            0, 
            0, 
            "Hello, world!".into(), 
        )
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class={format!("message mid{}", self.id)}>
              <div class="avatar"></div>
              <p class="content">{self.content.clone()}</p>
            </div>
        }
    }

}