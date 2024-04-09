use leptos::{view as html, *};
use leptos_router::A;

use crate::components::icons::user::UserIcon;

#[component]
pub fn Link<F, IV>(
  #[prop(into)] label: String,
  #[prop(into)] href: String,
  icon: F,
  #[prop(into)] shortcut_key: String,
) -> impl IntoView
where
  F: Fn() -> IV + 'static,
  IV: IntoView,
{
  let key_ui = String::from("[") + shortcut_key.as_str() + "]";

  html! {
    <div class="hover:bg-subtext0 hover:text-base w-full">
      <A class="text-xl flex items-center w-full space-x-2 p-2" href={href} exact=true>
        <span aria-hidden="true">{icon()}</span>
        <span>{label}</span>
        <span class="flex-1 text-right text-lg uppercase">
         {key_ui}
        </span>
      </A>
    </div>
  }
}

#[component]
pub fn Home() -> impl IntoView {
  html! {
     <div class="flex flex-col items-center justify-center h-full space-y-5">
       <h1 class="text-4xl font-bold text-white">Alessio Marchi</h1>
       <nav class="flex flex-col items-center justify-center min-w-[250px]">
        <Link label={"About"} href={"/about"} icon=UserIcon shortcut_key='a' />
        <Link label={"Projects"} href={"/projects"} icon=UserIcon shortcut_key='p' />
        <Link label={"Blog"} href={"/blog"} icon=UserIcon shortcut_key='b' />
      </nav>
    </div>
  }
}
