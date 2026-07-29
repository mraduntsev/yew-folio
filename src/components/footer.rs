use yew::prelude::*;
use crate::content::footer_data::{SOCIAL_LINKS, COPYRIGHT};
use crate::ui::Icon;

#[function_component(Footer)]
pub fn footer() -> Html {
  html! {
        <footer class="border-t border-ink-200 dark:border-ink-800 py-10">
            <div class="max-w-6xl mx-auto px-6 flex flex-col md:flex-row items-center justify-between gap-4">
                <div class="flex items-center gap-2 text-sm text-ink-500">
                    <div class="w-6 h-6 rounded-md bg-gradient-to-br from-rust-500 to-rust-700 flex items-center justify-center text-white font-mono font-bold text-[10px]">
                        {"</>"}
                    </div>
                    <span>{ COPYRIGHT }</span>
                </div>

                <div class="flex items-center gap-5 text-sm text-ink-500">

                {
                    SOCIAL_LINKS.iter().map(|link| html! {
                    <a
                        key={link.label}
                        href={link.url}
                        class="hover:text-rust-500 transition flex items-center gap-1"
                        download={if link.label == "Resume" { Some("") } else { None }}
                            >
                                if let Some(icon_name) = link.icon {
                                    <Icon name={icon_name} size=12 color="currentColor" stroke_width="2.5" />
                                }
                                { link.label }
                    </a>
                }).collect::<Html>()
                }
                </div>
            </div>
        </footer>
    }
}