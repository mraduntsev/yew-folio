use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::{JsCast, JsValue, closure::Closure};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, window};
use yew::{Html, function_component, html, use_effect_with, use_node_ref};
use crate::content::Particle;


impl Particle {
    fn random(canvas_width: f64, canvas_height: f64) -> Self {
        let size = js_sys::Math::random() * 2.0 + 1.0; // 1..3
        let x = js_sys::Math::random() * canvas_width;
        let y = js_sys::Math::random() * canvas_height;
        let speed_x = (js_sys::Math::random() * 2.0 - 1.0) * 0.5; // -0.5..0.5
        let speed_y = (js_sys::Math::random() * 2.0 - 1.0) * 0.5;
        Self {
            x,
            y,
            size,
            speed_x,
            speed_y,
        }
    }

    fn update(&mut self, canvas_width: f64, canvas_height: f64) {
        self.x += self.speed_x;
        self.y += self.speed_y;

        if self.x > canvas_width {
            self.x = 0.0;
        }
        if self.x < 0.0 {
            self.x = canvas_width;
        }
        if self.y > canvas_height {
            self.y = 0.0;
        }
        if self.y < 0.0 {
            self.y = canvas_height;
        }
    }

    fn draw(&self, ctx: &CanvasRenderingContext2d) {
        ctx.set_fill_style_str("rgba(255, 255, 255, 0.8)");
        ctx.begin_path();
        ctx.arc(self.x, self.y, self.size, 0.0, std::f64::consts::PI * 2.0)
            .expect("arc failed");
        ctx.close_path();
        ctx.fill();
    }
}

#[function_component(ParticleCanvas)]
pub fn particle_canvas() -> Html {
    let canvas_ref = use_node_ref();

    {
        let canvas_ref = canvas_ref.clone();
        use_effect_with((), move |_| {
            let cleanup: Rc<RefCell<Option<Closure<dyn FnMut()>>>> = Rc::new(RefCell::new(None));

            if let Some(canvas) = canvas_ref.cast::<HtmlCanvasElement>() {
                let ctx = canvas
                    .get_context("2d")
                    .unwrap()
                    .unwrap()
                    .dyn_into::<CanvasRenderingContext2d>()
                    .unwrap();

                let window = window().unwrap();

                let (w, h) = {
                    let w = window.inner_width().unwrap().as_f64().unwrap();
                    let h = window.inner_height().unwrap().as_f64().unwrap();
                    canvas.set_width(w as u32);
                    canvas.set_height(h as u32);
                    (w, h)
                };

                let particles = Rc::new(RefCell::new(
                    (0..100).map(|_| Particle::random(w, h)).collect::<Vec<_>>(),
                ));

                let animation_id: Rc<RefCell<Option<i32>>> = Rc::new(RefCell::new(None));

                let animate_closure: Rc<RefCell<Option<Closure<dyn FnMut()>>>> =
                    Rc::new(RefCell::new(None));

                let animate = {
                    let canvas = canvas.clone();
                    let ctx = ctx.clone();
                    let particles = particles.clone();
                    let animation_id = animation_id.clone();
                    let window = window.clone();
                    let animate_ref = animate_closure.clone();

                    Closure::wrap(Box::new(move || {
                        ctx.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
                        let w = canvas.width() as f64;
                        let h = canvas.height() as f64;
                        for p in particles.borrow_mut().iter_mut() {
                            p.update(w, h);
                            p.draw(&ctx);
                        }

                        if let Some(ref animate_fn) = *animate_ref.borrow() {
                            let next_id = window
                                .request_animation_frame(animate_fn.as_ref().unchecked_ref())
                                .unwrap();
                            *animation_id.borrow_mut() = Some(next_id);
                        }
                    }) as Box<dyn FnMut()>)
                };

                *animate_closure.borrow_mut() = Some(animate);

                if let Some(ref animate_fn) = *animate_closure.borrow() {
                    let init_id = window
                        .request_animation_frame(animate_fn.as_ref().unchecked_ref())
                        .unwrap();
                    *animation_id.borrow_mut() = Some(init_id);
                }

                let on_resize: Rc<Closure<dyn FnMut(web_sys::Event)>> = {
                    let canvas = canvas.clone();
                    let particles = particles.clone();
                    let window = window.clone();
                    Rc::new(Closure::wrap(Box::new(move |_: web_sys::Event| {
                        let w = window.inner_width().unwrap().as_f64().unwrap();
                        let h = window.inner_height().unwrap().as_f64().unwrap();
                        canvas.set_width(w as u32);
                        canvas.set_height(h as u32);
                        let mut vec = particles.borrow_mut();
                        vec.clear();
                        for _ in 0..100 {
                            vec.push(Particle::random(w, h));
                        }
                    }) as Box<dyn FnMut(_)>))
                };

                window
                    .add_event_listener_with_callback(
                        "resize",
                        on_resize.as_ref().as_ref().unchecked_ref(),
                    )
                    .unwrap();

                let cleanup_animation_id = animation_id.clone();
                let cleanup_on_resize = on_resize.clone();
                let cleanup_animate_closure = animate_closure.clone();
                let cleanup_window = window.clone();

                *cleanup.borrow_mut() = Some(Closure::once(move || {
                    if let Some(id) = *cleanup_animation_id.borrow() {
                        cleanup_window.cancel_animation_frame(id).unwrap();
                    }
                    cleanup_window
                        .remove_event_listener_with_callback(
                            "resize",
                            cleanup_on_resize.as_ref().as_ref().unchecked_ref(),
                        )
                        .unwrap();
                    drop(cleanup_on_resize);
                    drop(cleanup_animate_closure);
                }));
            }

            move || {
                if let Some(c) = cleanup.borrow_mut().take() {
                    let func: &js_sys::Function = c.as_ref().unchecked_ref();
                    let _ = func.call0(&JsValue::NULL);
                }
            }
        });
    }

    html! {
        <canvas
            ref={canvas_ref}
            id="particleCanvas"
            style="position: fixed; top: 0; left: 0; width: 100%; height: 100%; pointer-events: none; z-index: -1;"
        />
    }
}
