use yew::prelude::*;

use crate::{
    components::ContactChannel, content::load_locale, hooks::use_language, ui::SectionHeader,
};

#[function_component(Contact)]
pub fn contact() -> Html {
    let lang = use_language().current;
    let contact = load_locale(lang).contact;

    html! {
        <section id="contact" class="py-24 relative">
            <div class="max-w-5xl mx-auto px-6">
                <SectionHeader
                    number={contact.number}
                    title={contact.title}
                    description={contact.description}
                    centered={contact.centered}
                />

                <div class="reveal grid sm:grid-cols-2 lg:grid-cols-4 gap-4 mb-10">
                              {
                                contact.channels.iter().map(|ch| html! {
                                    <ContactChannel
                                        key={ch.label.clone()}
                                        label={ch.label.clone()}
                                        value={ch.value.clone()}
                                        icon_name={ch.icon.clone()}
                                        url={ch.url.clone()}
                                        description={ch.description.clone()}
                                    />
                                }).collect::<Html>()
                            }
                </div>
            </div>
        </section>
    }
}
