use makepad_draw_2d::*;
use makepad_widgets::text_input::TextInputFrameRefExt;
use makepad_widgets::*;

live_design! {
    import makepad_widgets::label::Label;
    import makepad_widgets::text_input::TextInput;

      App = {{App}} {
        // The `ui` field on the struct `App` defines a frame widget. Frames are used as containers
        // for other widgets. Since the `ui` property on the DSL object `App` corresponds with the
        // `ui` field on the Rust struct `App`, the latter will be initialized from the DSL object
        // here below.
        ui: {
            // The `layout` property determines how child widgets are laid out within a frame. In
            // this case, child widgets flow downward, with 20 pixels of spacing in between them,
            // and centered horizontally with respect to the entire frame.
            //
            // Because the child widgets flow downward, vertical alignment works somewhat
            // differently. In this case, children are centered vertically with respect to the
            // remainder of the frame after the previous children have been drawn.
            layout: {
                flow: Right,
                spacing: 20,
                align: {
                    x: 0.5,
                    y: 0.5
                }
            },
            // The `walk` property determines how the frame widget itself is laid out. In this
            // case, the frame widget takes up the entire window.
            walk: {
                width: Fill,
                height: Fill
            },
            bg: {
                shape: Solid

                // The `fn pixel(self) -> vec4` syntax is used to define a property named `pixel`,
                // the value of which is a shader. We use our own custom DSL to define shaders. It's
                // syntax is *mostly* compatible with GLSL, although there are some differences as
                // well.
                fn pixel(self) -> vec4 {
                    // Within a shader, the `self.geom_pos` syntax is used to access the `geom_pos`
                    // attribute of the shader. In this case, the `geom_pos` attribute is built in,
                    // and ranges from 0 to 1. over x and y of the rendered rectangle
                    return mix(#7, #3, self.geom_pos.y);
                }
            }

           input_celsius = <TextInput> {
                // walk: {width:100, height:30},
                text: "Input Celsius"
            }

            label_celsius = <Label> {
                walk: { width: 60, height: 50 }
                align: {
                    x: 0.5,
                    y: 0.5
                }
                label: {
                    color: #0f0
                },
                text: "Celsius = "
            }

            input_fahrenheit = <TextInput> {
                text: "Input Fahrenheit"
            }

            label_fahrenheit = <Label> {
                walk: { width: 60, height: 50 }
                align: {
                    x: 0.5,
                    y: 0.5
                }
                label: {
                    color: #0ff
                },
                text: "Fahrenheit"
            }
        }
    }
}

// This main_app macro generates the code necessary to initialize and run your application.
//
// This code is almost always the same between different applications, so it is convenient to use a
// macro for it. The two main tasks that this code needs to carry out are: initializing both the
// main application struct (`App`) and the global context object (`Cx`), and setting up event
// handling. On desktop, this means creating and running our own event loop from a fn main(). On web, this means
// creating an event handler function that the browser event loop can call into.
main_app!(App);

// The main application struct.
//
// The #[derive(Live, LiveHook)] attribute implements a bunch of traits for this struct that enable
// it to interact with the Makepad runtime. Among other things, this enables the Makepad runtime to
// initialize the struct from a DSL object.
#[derive(Live, LiveHook)]
pub struct App {
    // A chromeless window for our application. Used to contain our frame widget.
    window: BareWindow,
    // A frame widget. Used to contain our button and label.
    ui: FrameRef,

    // The value for our counter.
    //
    // The #[rust] attribute here is used to indicate that this field should *not* be initialized
    // from a DSL object, even when a corresponding property exists.
    #[rust]
    c_value: usize,
    f_value: usize,
}

impl App {
    pub fn live_design(cx: &mut Cx) {
        makepad_widgets::live_design(cx);
    }

     pub fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        if let Event::Draw(event) = event {
            // This is a draw event, so create a draw context and use that to draw our application.
            let mut draw_cx = Cx2d::new(cx, event);
            return self.draw(&mut draw_cx);
        }

        // Forward the event to the window.
        self.window.handle_event(cx, event);

        // Forward the event to the frame. In this case, handle_event returns a list of actions.
        // Actions are similar to events, except that events are always forwarded downward to child
        // widgets, while actions are always returned back upwards to parent widgets.
        let actions = self.ui.handle_event(cx, event);

        // if self.ui.get_text_input(id!(input_celsius)).clicked(&actions) {
        //     println!("C Focus In");
        //     // let x = self.ui.get_text_input(id!(input_celsius)).selected_text();
        //     // println!("C Focus value {}", x);
        // }
        if let res = self.ui.get_text_input(id!(input_celsius)).changed(&actions) {
            // println!("C Changed {}", res);
            if ! res.is_empty() {
                self.f_value =(res.parse::<usize>().unwrap()) * 9 / 5 + 32;
                // println!("New F={}", self.f_value);

                let inp_f = self.ui.get_text_input(id!(input_fahrenheit));
                inp_f.set_text(&format!("{}", self.f_value));
                inp_f.redraw(cx);
            }
        };

        if let res = self.ui.get_text_input(id!(input_fahrenheit)).changed(&actions) {
            // println!("F Changed {}", res);
            if ! res.is_empty() {
                self.c_value =(res.parse::<usize>().unwrap()) * 9 / 5 + 32;
                // println!("New C={}", self.c_value);

                let inp_c = self.ui.get_text_input(id!(input_celsius));
                inp_c.set_text(&format!("{}", self.c_value));
                inp_c.redraw(cx);
            }
        };
    }

    // This is the immediate mode draw flow, as called above in response to the Draw event
    pub fn draw(&mut self, cx: &mut Cx2d) {
        // Indicate that we want to begin drawing to the window.
        if self.window.begin(cx).not_redrawing() {
            return;
        }

        // Draw the frame to the window.
        let _ = self.ui.draw(cx);

        // Indicate that we finished drawing to the window.
        self.window.end(cx);
    }
}
