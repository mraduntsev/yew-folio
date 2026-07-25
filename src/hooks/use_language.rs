use yew::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum Language {
    En,
    Ru,
}

#[derive(Clone, PartialEq)]
pub struct LanguageContext {
    pub current: Language,
    pub setter: UseStateSetter<Language>,
}

#[derive(Properties, PartialEq)]
pub struct LanguageProviderProps {
    pub children: Children,
}

#[function_component(LanguageProvider)]
pub fn language_provider(props: &LanguageProviderProps) -> Html {
    let language = use_state(|| Language::Ru);

    let context = LanguageContext {
        current: *language,
        setter: language.setter(),
    };

    html! {
        <ContextProvider<LanguageContext> context={context}>
            { for props.children.iter() }
        </ContextProvider<LanguageContext>>
    }
}

#[hook]
pub fn use_language() -> LanguageContext {
    use_context::<LanguageContext>().expect("LanguageContext not found")
}
