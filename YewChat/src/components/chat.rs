use serde::{Deserialize, Serialize};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_agent::{Bridge, Bridged};

use crate::services::event_bus::EventBus;
use crate::{services::websocket::WebsocketService, User};

pub enum Msg {
    HandleMsg(String),
    SubmitMessage,
}

#[derive(Deserialize)]
struct MessageData {
    from: String,
    message: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum MsgTypes {
    Users,
    Register,
    Message,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WebSocketMessage {
    message_type: MsgTypes,
    data_array: Option<Vec<String>>,
    data: Option<String>,
}

#[derive(Clone)]
struct UserProfile {
    name: String,
    avatar: String,
}

pub struct Chat {
    users: Vec<UserProfile>,
    chat_input: NodeRef,
    _producer: Box<dyn Bridge<EventBus>>,
    wss: WebsocketService,
    messages: Vec<MessageData>,
}

impl Component for Chat {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let (user, _) = ctx
            .link()
            .context::<User>(Callback::noop())
            .expect("context to be set");
        let wss = WebsocketService::new();
        let username = user.username.borrow().clone();

        let message = WebSocketMessage {
            message_type: MsgTypes::Register,
            data: Some(username.to_string()),
            data_array: None,
        };

        if let Ok(_) = wss
            .tx
            .clone()
            .try_send(serde_json::to_string(&message).unwrap())
        {
            log::debug!("message sent successfully");
        }

        Self {
            users: vec![],
            messages: vec![],
            chat_input: NodeRef::default(),
            wss,
            _producer: EventBus::bridge(ctx.link().callback(Msg::HandleMsg)),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::HandleMsg(s) => {
                let msg: WebSocketMessage = serde_json::from_str(&s).unwrap();
                match msg.message_type {
                    MsgTypes::Users => {
                        let users_from_message = msg.data_array.unwrap_or_default();
                        self.users = users_from_message
                            .iter()
                            .map(|u| UserProfile {
                                name: u.into(),
                                avatar: format!(
                                    "https://avatars.dicebear.com/api/adventurer-neutral/{}.svg",
                                    u
                                )
                                .into(),
                            })
                            .collect();
                        return true;
                    }
                    MsgTypes::Message => {
                        let message_data: MessageData =
                            serde_json::from_str(&msg.data.unwrap()).unwrap();
                        self.messages.push(message_data);
                        return true;
                    }
                    _ => {
                        return false;
                    }
                }
            }
            Msg::SubmitMessage => {
                let input = self.chat_input.cast::<HtmlInputElement>();
                if let Some(input) = input {
                    let message = WebSocketMessage {
                        message_type: MsgTypes::Message,
                        data: Some(input.value()),
                        data_array: None,
                    };
                    if let Err(e) = self
                        .wss
                        .tx
                        .clone()
                        .try_send(serde_json::to_string(&message).unwrap())
                    {
                        log::debug!("error sending to channel: {:?}", e);
                    }
                    input.set_value("");
                };
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let submit = ctx.link().callback(|_| Msg::SubmitMessage);

        html! {
            <div class="flex w-screen h-screen bg-gradient-to-br from-slate-900 via-emerald-900 to-teal-900 relative overflow-hidden">
                <div class="absolute inset-0 bg-gradient-to-tr from-emerald-500/10 to-cyan-500/10"></div>
                <div class="absolute top-0 left-1/4 w-96 h-96 bg-emerald-400/20 rounded-full blur-3xl animate-pulse"></div>
                <div class="absolute bottom-0 right-1/4 w-80 h-80 bg-teal-400/20 rounded-full blur-3xl animate-pulse" style="animation-delay: 2s"></div>
                
                <div class="flex-none w-80 h-screen bg-gradient-to-b from-emerald-800/80 to-teal-900/80 backdrop-blur-xl shadow-2xl border-r border-white/10 relative z-10">
                    <div class="bg-gradient-to-r from-emerald-600/30 to-teal-600/30 backdrop-blur-sm border-b border-white/20">
                        <div class="text-2xl font-bold text-white p-6">
                            {"Active Users"}
                            <div class="text-sm font-normal text-emerald-200 mt-1 flex items-center">
                                {format!("{} online", self.users.len())}
                                <div class="ml-2 w-2 h-2 bg-emerald-400 rounded-full animate-pulse"></div>
                            </div>
                        </div>
                    </div>
                    <div class="p-4 space-y-3 overflow-y-auto h-full pb-20">
                        {
                            self.users.clone().iter().enumerate().map(|(i, u)| {
                                let delay = format!("animation-delay: {}s", i as f32 * 0.1);
                                html!{
                                    <div class="flex items-center bg-white/10 backdrop-blur-sm rounded-2xl p-4 hover:bg-white/20 transition-all duration-500 transform hover:scale-105 border border-white/10 hover:border-emerald-400/30 group" style={delay}>
                                        <div class="relative">
                                            <img class="w-14 h-14 rounded-full border-3 border-emerald-300 shadow-xl group-hover:shadow-emerald-400/50" src={u.avatar.clone()} alt="avatar"/>
                                            <div class="absolute -bottom-1 -right-1 w-4 h-4 bg-gradient-to-r from-emerald-400 to-teal-500 rounded-full border-2 border-white animate-pulse"></div>
                                        </div>
                                        <div class="flex-grow ml-4">
                                            <div class="font-semibold text-white text-base group-hover:text-emerald-200 transition-colors">
                                                {u.name.clone()}
                                            </div>
                                            <div class="text-emerald-300 text-sm">
                                                {"Active now"}
                                            </div>
                                        </div>
                                        <div class="w-3 h-8 bg-emerald-400/30 rounded-full group-hover:bg-emerald-400/50 transition-colors"></div>
                                    </div>
                                }
                            }).collect::<Html>()
                        }
                    </div>
                </div>

                <div class="grow h-screen flex flex-col relative z-10">
                    <div class="bg-gradient-to-r from-emerald-700/80 to-teal-800/80 backdrop-blur-xl shadow-lg border-b border-white/10">
                        <div class="text-3xl font-bold text-white p-6 flex items-center justify-between">
                            {"Chat Hub"}
                            <div class="flex space-x-2">
                                <div class="w-3 h-3 bg-emerald-300 rounded-full animate-ping"></div>
                                <div class="w-3 h-3 bg-teal-300 rounded-full animate-ping" style="animation-delay: 0.5s"></div>
                                <div class="w-3 h-3 bg-cyan-300 rounded-full animate-ping" style="animation-delay: 1s"></div>
                            </div>
                        </div>
                    </div>

                    <div class="flex-1 overflow-y-auto p-8 space-y-6 bg-gradient-to-b from-slate-900/50 to-emerald-900/30">
                        {
                            self.messages.iter().enumerate().map(|(i, m)| {
                                let user = self.users.iter().find(|u| u.name == m.from).unwrap();
                                let bubble_gradient = if i % 2 == 0 {
                                    "from-emerald-500/20 to-teal-600/20 border-emerald-400/30"
                                } else {
                                    "from-teal-500/20 to-cyan-600/20 border-teal-400/30"
                                };
                                
                                html!{
                                    <div class="flex items-start max-w-4xl animate-fade-in">
                                        <div class="relative">
                                            <img class="w-12 h-12 rounded-full border-2 border-emerald-300 shadow-lg" src={user.avatar.clone()} alt="avatar"/>
                                            <div class="absolute -bottom-1 -right-1 w-3 h-3 bg-emerald-400 rounded-full border border-white animate-pulse"></div>
                                        </div>
                                        <div class={format!("ml-4 bg-gradient-to-br {} backdrop-blur-sm rounded-2xl rounded-tl-md shadow-xl p-5 max-w-lg border transform hover:scale-105 transition-all duration-300", bubble_gradient)}>
                                            <div class="font-semibold text-emerald-200 text-sm mb-2 flex items-center">
                                                {m.from.clone()}
                                                <div class="ml-2 w-2 h-2 bg-emerald-400 rounded-full"></div>
                                            </div>
                                            <div class="text-white font-medium">
                                                if m.message.ends_with(".gif") {
                                                    <img class="mt-3 rounded-xl shadow-lg max-w-full border-2 border-white/20" src={m.message.clone()}/>
                                                } else {
                                                    <p class="leading-relaxed">{m.message.clone()}</p>
                                                }
                                            </div>
                                        </div>
                                    </div>
                                }
                            }).collect::<Html>()
                        }
                    </div>

                    <div class="bg-gradient-to-r from-emerald-800/60 to-teal-900/60 backdrop-blur-xl border-t border-white/10 p-6">
                        <div class="flex items-center space-x-4 max-w-5xl mx-auto">
                            <div class="flex-1 relative group">
                                <input 
                                    ref={self.chat_input.clone()} 
                                    type="text" 
                                    placeholder="Share your thoughts..." 
                                    class="w-full py-4 px-6 bg-white/10 backdrop-blur-sm rounded-2xl border border-white/20 focus:border-emerald-400 focus:outline-none focus:ring-4 focus:ring-emerald-400/30 transition-all duration-300 text-white placeholder-emerald-200/70 font-medium text-lg group-hover:bg-white/15" 
                                    name="message" 
                                    required=true 
                                />
                                <div class="absolute inset-0 rounded-2xl bg-gradient-to-r from-emerald-400/10 to-teal-400/10 opacity-0 group-hover:opacity-100 transition-opacity duration-300 pointer-events-none"></div>
                            </div>
                            <button 
                                onclick={submit} 
                                class="w-14 h-14 bg-gradient-to-r from-emerald-500 to-teal-600 hover:from-emerald-400 hover:to-teal-500 rounded-2xl flex justify-center items-center shadow-2xl hover:shadow-emerald-500/50 transition-all duration-300 transform hover:scale-110 hover:rotate-6 active:scale-95"
                            >
                                <svg viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" class="w-6 h-6 fill-white transform translate-x-0.5">
                                    <path d="M0 0h24v24H0z" fill="none"></path>
                                    <path d="M2.01 21L23 12 2.01 3 2 10l15 2-15 2z"></path>
                                </svg>
                            </button>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}