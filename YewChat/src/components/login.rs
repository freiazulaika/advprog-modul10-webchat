use web_sys::HtmlInputElement;
use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;
use crate::User;

#[function_component(Login)]
pub fn login() -> Html {
    let username = use_state(|| String::new());
    let user = use_context::<User>().expect("No context found.");

    let oninput = {
        let current_username = username.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            current_username.set(input.value());
        })
    };

    let onclick = {
        let username = username.clone();
        let user = user.clone();
        Callback::from(move |_| *user.username.borrow_mut() = (*username).clone())
    };

    html! {
       <div class="bg-gradient-to-br from-emerald-600 via-teal-700 to-cyan-800 flex w-screen min-h-screen relative overflow-hidden">
            <div class="absolute inset-0 bg-gradient-to-tr from-black/20 to-transparent"></div>
            <div class="absolute top-20 left-20 w-72 h-72 bg-emerald-400/20 rounded-full blur-3xl"></div>
            <div class="absolute bottom-20 right-20 w-96 h-96 bg-cyan-400/20 rounded-full blur-3xl"></div>
            
            <div class="container mx-auto flex flex-col justify-center items-center p-8 relative z-10">
                <div class="bg-white/10 backdrop-blur-2xl rounded-3xl p-12 shadow-2xl border border-white/20 hover:bg-white/15 transition-all duration-500">
                    <h1 class="text-5xl font-bold text-white text-center mb-4 bg-gradient-to-r from-white to-emerald-100 bg-clip-text text-transparent">{"Welcome Back"}</h1>
                    <p class="text-emerald-100 text-center mb-12 text-lg font-medium">{"Ready to dive into conversations?"}</p>
                    
                    <form class="w-80">
                        <div class="relative group">
                            <input 
                                {oninput} 
                                class="w-full p-6 border-0 text-white bg-white/10 backdrop-blur-sm placeholder-emerald-200/70 focus:outline-none focus:ring-4 focus:ring-emerald-400/50 rounded-2xl transition-all duration-300 text-lg font-medium focus:bg-white/20 group-hover:bg-white/15" 
                                placeholder="Enter your username" 
                            />
                            <div class="absolute inset-0 rounded-2xl bg-gradient-to-r from-emerald-400/20 to-cyan-400/20 opacity-0 group-hover:opacity-100 transition-opacity duration-300 pointer-events-none"></div>
                        </div>
                        
                        <Link<Route> to={Route::Chat}> 
                            <button 
                                {onclick} 
                                disabled={username.len()<1} 
                                class="w-full mt-6 py-4 px-8 bg-gradient-to-r from-emerald-500 to-teal-600 hover:from-emerald-400 hover:to-teal-500 disabled:from-gray-500 disabled:to-gray-600 text-white font-bold text-lg rounded-2xl transition-all duration-300 transform hover:scale-105 hover:shadow-2xl disabled:scale-100 disabled:cursor-not-allowed shadow-xl"
                            >
                                {"Let's Chat"}
                            </button>
                        </Link<Route>>
                    </form>
                </div>
                
                <div class="mt-8 flex space-x-3">
                    <div class="w-2 h-2 bg-emerald-300 rounded-full animate-pulse"></div>
                    <div class="w-2 h-2 bg-teal-300 rounded-full animate-pulse" style="animation-delay: 0.5s"></div>
                    <div class="w-2 h-2 bg-cyan-300 rounded-full animate-pulse" style="animation-delay: 1s"></div>
                </div>
            </div>
        </div>
    }
}