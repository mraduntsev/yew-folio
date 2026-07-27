use yew::prelude::*;
use crate::content::get_hero_data;
use crate::ui::Button;
use crate::ui::Icon;
use crate::ui::Badge;

#[function_component(Hero)]
pub fn hero() -> Html {
    let data = get_hero_data();

    html! {
        <section id="hero" class="relative pt-32 pb-24 overflow-hidden">
            <div class="absolute inset-0 grid-bg opacity-60"></div>
            <div class="absolute inset-0 hero-glow"></div>

            <div class="relative max-w-6xl mx-auto px-6 grid md:grid-cols-5 gap-12 items-center">
                <div class="md:col-span-3 space-y-6 animate-fade-up">
                    <Badge variant="dot" color="emerald">
                        { "Available for new projects" }
                    </Badge>

                    <h1 class="text-5xl md:text-7xl font-bold tracking-tight leading-[1.05]">
                        { "Hi, I'm " }<span class="gradient-text">{ data.name }</span>{ "." }
                        <br/>
                        { "I build " }<span class="font-mono cursor">{ "fast systems" }</span>
                    </h1>

                    <p class="text-lg text-ink-600 dark:text-ink-300 max-w-xl leading-relaxed">
                        { data.description }
                    </p>

                    <div class="flex flex-wrap gap-3 pt-2">
                        <Button variant="primary" href={data.links.projects} right_icon="arrow-right">
                            { "View projects" }
                        </Button>
                        <Button variant="outline" href={data.links.resume} left_icon="download">
                            { "Download CV" }
                        </Button>
                        <Button variant="ghost" href={data.links.contact}>
                            { "Get in touch" }
                        </Button>
                    </div>

                    <div class="flex items-center gap-6 pt-6 text-sm text-ink-500 dark:text-ink-400 font-mono">
                        { for data.stats.iter().map(|(num, label)| html! {
                            <>
                                <div>
                                    <span class="text-2xl font-bold text-ink-900 dark:text-ink-100">{ *num }</span>
                                    <span class="ml-1">{ *label }</span>
                                </div>
                                <div class="w-px h-8 bg-ink-200 dark:bg-ink-800"></div>
                            </>
                        }) }
                    </div>
                </div>

                <div class="md:col-span-2 animate-float">
                    <TerminalCard />
                </div>
            </div>
        </section>
    }
}

#[function_component]
fn TerminalCard() -> Html {
    html! {
        <div class="relative">
            <div class="absolute -inset-4 bg-gradient-to-br from-rust-500/30 to-rust-700/10 rounded-3xl blur-2xl"></div>
            <div class="relative rounded-2xl overflow-hidden border border-ink-200 dark:border-ink-800 bg-white dark:bg-ink-900 shadow-2xl">
                <div class="flex items-center gap-2 px-4 py-3 border-b border-ink-200 dark:border-ink-800 bg-ink-50 dark:bg-ink-900/50">
                    <span class="w-3 h-3 rounded-full bg-red-400"></span>
                    <span class="w-3 h-3 rounded-full bg-yellow-400"></span>
                    <span class="w-3 h-3 rounded-full bg-green-400"></span>
                    <span class="ml-2 text-xs font-mono text-ink-500">{ "~/raduntsev — zsh" }</span>
                </div>
                <div class="p-5 font-mono text-sm leading-relaxed bg-ink-900 text-ink-100">
                    <div><span class="text-rust-400">{ "$" }</span> <span class="text-emerald-400">{ "whoami" }</span></div>
                    <div class="text-ink-300">{ "raduntsev — backend engineer" }</div>
                    <div class="mt-2"><span class="text-rust-400">{ "$" }</span> <span class="text-emerald-400">{ "cat" }</span> { "stack.toml" }</div>
                    <div class="text-ink-300">{ "[languages]" }</div>
                    <div class="text-ink-300 pl-3">{ "rust  = " }<span class="text-rust-400">{ "\"primary\"" }</span></div>
                    <div class="text-ink-300 pl-3">{ "cpp   = " }<span class="text-rust-400">{ "\"daily\"" }</span></div>
                    <div class="text-ink-300 pl-3">{ "python = " }<span class="text-rust-400">{ "\"scripting\"" }</span></div>
                    <div class="mt-2"><span class="text-rust-400">{ "$" }</span> <span class="text-emerald-400">{ "echo" }</span> { "$FOCUS" }</div>
                    <div class="text-ink-300">{ "low-latency · reliability · clean APIs" }</div>
                    <div class="mt-2"><span class="text-rust-400">{ "$" }</span> <span class="cursor"></span></div>
                </div>
            </div>
        </div>
    }
}