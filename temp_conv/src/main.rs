use makepad_draw_2d::*;
use makepad_widgets::text_input::TextInputFrameRefExt;
use makepad_widgets::*;

live_design! {
    import makepad_widgets::label::Label;
    import makepad_widgets::text_input::TextInput;

      App = {{App}} {

        ui: {

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

                fn pixel(self) -> vec4 {
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
    c_value: String,
    f_value: String,
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

        let res = self.ui.get_text_input(id!(input_celsius)).changed(&actions);
        match res.parse::<i32>() {
            Ok(number) => {
                self.f_value = (number * 9 / 5 + 32).to_string();
                println!("F={}", self.f_value);
                let inp_f = self.ui.get_text_input(id!(input_fahrenheit));
                inp_f.set_text(&format!("{}", self.f_value));
                inp_f.redraw(cx);
            }
            Err(_) => {
                // println!("Invalid input. Please enter an integer.");
            }
        }

        let res = self.ui.get_text_input(id!(input_fahrenheit)).changed(&actions);
        match res.parse::<i32>() {
            Ok(number) => {
                self.c_value = ((number - 32) * 5/9).to_string();
                println!("C={}",  self.c_value);
                let inp_c = self.ui.get_text_input(id!(input_celsius));
                inp_c.set_text(&format!("{}", self.c_value));
                inp_c.redraw(cx);
            }
            Err(_) => {
                // println!("Invalid input. Please enter an integer.");
            }
        }

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
