use makepad_draw_2d::*;
use makepad_widgets;
use makepad_widgets::*;
use text_input::{TextInputAction};

live_design! {
    import makepad_widgets::label::Label;
    import makepad_widgets::text_input::TextInput;

    App = {{App}} {
        // The `ui` field on the struct `App` defines a frame widget. Frames are used as containers
        // for other widgets. Since the `ui` property on the DSL object `App` corresponds with the
        // `ui` field on the Rust struct `App`, the latter will be initialized from the DSL object
        // here below.
        // label_text: {
        //     color: #9
        // }

        // label_walk: {
        //     margin: {left: 4.0, top: 3.0}
        //     width: Fill,
        //     height: Fill
        // }

        // label_align: {
        //     y: 0.0
        // }

        // input_fahrenheit: {
        //     cursor_margin_bottom: 3.0,
        //     cursor_margin_top: 4.0,
        //     select_pad_edges: 3.0
        //     cursor_size: 2.0,
        //     empty_message: "0",
        //     numeric_only: true,
        //     // bg: {
        //     //     shape: None
        //     //     color: #f00
        //     //     radius: 2
        //     // },
        //     // layout: {
        //     //     padding: 0,
        //     //     align: {y: 0.}
        //     // },
        //     walk: {
        //         margin: {top: 3, right: 5}
        //     }
        // }

        // input_celsius = <TextInput> {
        //     // walk: { width: 100, height: 50 }
        //     cursor_margin_bottom: 3.0,
        //     cursor_margin_top: 4.0,
        //     select_pad_edges: 3.0
        //     cursor_size: 4.0,
        //     empty_message: "0",
        //     numeric_only: true,
        //     bg: {
        //         shape: None
        //         color: #0f0
        //         radius: 2
        //     },
        //     layout: {
        //         padding: 20,
        //         align: {y: 0}
        //     },
        //     walk: {
        //         margin: {top: 3, right: 5}
        //     }
        //     text: "XXXXXXXXXXXXXXXX"
        //     label: "Blah"
        //     draw_label: {
        //         text_style:
        //             {
        //                 // font: {path: d "crate://makepad-widgets/resources/IBMPlexSans-SemiBold.ttf"},
        //                 font_size: (20)
        //             }
        //     }
        // }

        // input_celsius2 = <TextInput> {
        //     walk: {
        //         margin: {top: 3, right: 5}
        //     }
        //     text: "Celsius"
        //     label: "Blah"
        // }

        input_celsius = <TextInput> {
            // walk: {width:100, height:30},
            walk: {
                width: 100,
                height: 50
            },
            text: "Input Celsius"
        }

        input_fahrenheit = <TextInput> {
            walk: {
                width: 100,
                height: 50
            },
            text: "Input Fahrenheit"
        }

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
                spacing: 50,
                align: {
                    x: 0.5,
                    y: 0.5
                }
            },
            // The `walk` property determines how the frame widget itself is laid out. In this
            // case, the frame widget takes up the entire window.
            walk: {
                width: 200,
                height: 50
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
                    return mix(#4, #4, self.geom_pos.y);
                }
            }

            // The `name:` syntax is used to define fields, i.e. properties for which there are
            // corresponding struct fields. In contrast, the `name =` syntax is used to define
            // instance properties, i.e. properties for which there are no corresponding struct
            // fields. Note that fields and instance properties use different namespaces, so you
            // can have both a field and an instance property with the same name.
            //
            // Widgets can hook into the Makepad runtime with custom code and determine for
            // themselves how they want to handle instance properties. In the case of frame widgets,
            // they simply iterate over their instance properties, and use them to instantiate their
            // child widgets.
            // input_celsius = <TextInput> {
            //     // walk: {width:100, height:30},
            //     walk: {
            //         width: 100,
            //         height: 50
            //     },
            //     text: "Input Celsius"
            // }

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

            // input_fahrenheit = <TextInput> {
            //     walk: {
            //         width: 100,
            //         height: 50
            //     },
            //     text: "Input Fahrenheit"
            // }

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

    ui: FrameRef,

    label_walk: Walk,
    label_align: Align,
    label_text: DrawText,
    label: String,
    label_celsius: Label,

    input_celsius: TextInput,

    input_fahrenheit: TextInput,

    label_fahrenheit: Label,

    // The value for our counter.
    //
    // The #[rust] attribute here is used to indicate that this field should *not* be initialized
    // from a DSL object, even when a corresponding property exists.
    // #[rust]
    // counter: usize,
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

        // for action in self.input_celsius.handle_event(cx, event) {
        for action in self.input_celsius.handle_event(cx, event) {
            match action {
                TextInputAction::KeyFocus => {
                    println!("C Focus In");
                }
                TextInputAction::KeyFocusLost => {
                    // self.animate_state(cx, id!(focus.off));
                    println!("C Focus Out");
                    self.update_input_fahrenheit(cx);
                }
                TextInputAction::Return(value) => {
                    println!("C Return {}", value);
                    // if let Ok(v) = value.parse::<f64>() {
                    //     self.set_internal(v.max(self.min).min(self.max));
                    // }
                    self.update_input_fahrenheit(cx);
                    // dispatch_action(cx, a::TextSlide(self.to_external()));
                }
                TextInputAction::Escape => {
                    println!("Escape");
                    // self.update_input_fahrenheit(cx);
                }
                _ => ()
            }
        };

        for action in self.input_fahrenheit.handle_event(cx, event) {
            match action {
                TextInputAction::KeyFocus => {
                    // self.animate_state(cx, id!(focus.on));
                    println!("F Focus In");
                }
                TextInputAction::KeyFocusLost => {
                    // self.animate_state(cx, id!(focus.off));
                    println!("F Focus Out");
                    self.update_input_celsius(cx);
                }
                TextInputAction::Return(value) => {
                    println!("F Return {}", value);
                    // if let Ok(v) = value.parse::<f64>() {
                    //     self.set_internal(v.max(self.min).min(self.max));
                    // }
                    self.update_input_celsius(cx);
                    // dispatch_action(cx, a::TextSlide(self.to_external()));
                }
                TextInputAction::Escape => {
                    println!("Escape");
                    // self.update_input_celsius(cx);
                }
                _ => ()
            }
        };
    }

    // This is the immediate mode draw flow, as called above in response to the Draw event
    pub fn draw(&mut self, cx: &mut Cx2d) {
        // Indicate that we want to begin drawing to the window.
        if self.window.begin(cx).not_redrawing() {
            return;
        }

        if let Some(dw) = cx.defer_walk(self.label_walk) {
            //, (self.value*100.0) as usize);
            self.input_celsius.draw_walk(cx, self.input_celsius.get_walk());
            // self.input_fahrenheit.draw_walk(cx, self.input_fahrenheit.get_walk());
            self.label_text.draw_walk(cx, dw.resolve(cx), self.label_align, &self.label);
        }
        // Draw the frame to the window.
        // self.input_celsius.draw_walk(cx, self.input_celsius.get_walk());
        let _ = self.ui.draw(cx);
        self.input_fahrenheit.draw_walk(cx, self.input_fahrenheit.get_walk());

        // let _lab_c = self.label_celsius.draw_widget(cx, self.label_celsius.get_walk());
        // let _lab_f = self.label_fahrenheit.draw_widget(cx, self.label_fahrenheit.get_walk());
        // let _lab_f1 = self.label_fahrenheit.draw_walk_widget(cx);

        // Indicate that we finished drawing to the window.
        self.window.end(cx);
    }

    pub fn update_input_fahrenheit(&mut self, cx: &mut Cx) {
        // let e = self.to_external();
        // self.text_input.text = match self.precision{
        //     0=>format!("{:.0}",e),
        //     1=>format!("{:.1}",e),
        //     2=>format!("{:.2}",e),
        //     3=>format!("{:.3}",e),
        //     4=>format!("{:.4}",e),
        //     5=>format!("{:.5}",e),
        //     6=>format!("{:.6}",e),
        //     7=>format!("{:.7}",e),
        //     _=>format!("{}",e)
        // };
        // self.text_input.select_all();
        // self.text_input.redraw(cx)

        let f_value = (self.input_celsius.text.parse::<i32>().unwrap() * 9 / 5 + 32).to_string();
        println!("f_value={}", f_value);

        // let label = self.ui.get_label(id!(label_fahrenheit));
        // label.set_text(&format!("{}", f_value));
        // label.redraw(cx);

        self.input_fahrenheit.select_all();
        self.input_fahrenheit.redraw(cx);
        self.input_fahrenheit.replace_text(&f_value);

    }


    pub fn update_input_celsius(&mut self, cx: &mut Cx) {
        // let e = self.to_external();
        // self.text_input.text = match self.precision{
        //     0=>format!("{:.0}",e),
        //     1=>format!("{:.1}",e),
        //     2=>format!("{:.2}",e),
        //     3=>format!("{:.3}",e),
        //     4=>format!("{:.4}",e),
        //     5=>format!("{:.5}",e),
        //     6=>format!("{:.6}",e),
        //     7=>format!("{:.7}",e),
        //     _=>format!("{}",e)
        // };
        // self.text_input.select_all();
        // self.text_input.redraw(cx)

        let c_value = (self.input_fahrenheit.text.parse::<i32>().unwrap()*5/9-32).to_string();
        println!("c_value={}", c_value);

        self.input_celsius.select_all();
        self.input_celsius.redraw(cx);
        self.input_celsius.replace_text(&c_value);
    }

}

