use yew::prelude::*;
use crate::ui::SectionHeader;
use crate::components::ContactChannel;
use crate::content::{CONTACTS_SECTION, get_contact_channels};

#[function_component(Contact)]
pub fn contact() -> Html {
    let channels = get_contact_channels();

    html! {
        <section id="contact" class="py-24 relative">
            <div class="max-w-5xl mx-auto px-6">
                <SectionHeader
                    number={CONTACTS_SECTION.number}
                    title={CONTACTS_SECTION.title}
                    description={CONTACTS_SECTION.description}
                    centered={CONTACTS_SECTION.centered}
                />

                <div class="reveal grid sm:grid-cols-2 lg:grid-cols-4 gap-4 mb-10">
                    { for channels.into_iter().map(|ch| {
                        html! {
                            <ContactChannel
                                key={ch.label}
                                label={ch.label}
                                value={ch.value}
                                icon_name={ch.icon}
                                url={ch.url}
                                description={ch.description}
                            />
                        }
                    }) }
                </div>
            </div>
        </section>
    }
}