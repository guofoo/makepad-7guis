use makepad_draw_2d::*;
use makepad_widgets;
use makepad_widgets::*;
use text_input::{TextInputAction};

live_design! {
    import makepad_widgets::label::Label;
    import makepad_widgets::text_input::TextInput;

    App = {{App}} {
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

                fn pixel(self) -> vec4 {
                    return mix(#4, #4, self.geom_pos.y);
                }
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


main_app!(App);

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
                    // println!("C Focus In");
                }
                TextInputAction::KeyFocusLost => {
                    // println!("C Focus Out");
                    self.update_input_fahrenheit(cx);
                }
                TextInputAction::Return(value) => {
                    // println!("C Return {}", value);
                    self.update_input_fahrenheit(cx);
                }
                TextInputAction::Escape => {
                    // println!("Escape");
                }
                _ => ()
            }
        };

        for action in self.input_fahrenheit.handle_event(cx, event) {
            match action {
                TextInputAction::KeyFocus => {
                    // println!("F Focus In");
                }
                TextInputAction::KeyFocusLost => {
                    // println!("F Focus Out");
                    self.update_input_celsius(cx);
                }
                TextInputAction::Return(value) => {
                    // println!("F Return {}", value);
                    self.update_input_celsius(cx);
                }
                TextInputAction::Escape => {
                    // println!("Escape");
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
        match self.input_celsius.text.parse::<i32>() {
            Ok(number) => {
                let f_value = (number * 9 / 5 + 32).to_string();
                println!("C={}", f_value);
                self.input_fahrenheit.select_all();
                self.input_fahrenheit.redraw(cx);
                self.input_fahrenheit.replace_text(&f_value);
            }
            Err(_) => {
                println!("Invalid input. Please enter an integer.");
            }
        }
    }

    pub fn update_input_celsius(&mut self, cx: &mut Cx) {
        match self.input_fahrenheit.text.parse::<i32>() {
            Ok(number) => {
                let c_value = ((number - 32) * 5/9).to_string();
                println!("C={}", c_value);
                self.input_celsius.select_all();
                self.input_celsius.redraw(cx);
                self.input_celsius.replace_text(&c_value);
            }
            Err(_) => {
                println!("Invalid input. Please enter an integer.");
            }
        }
    }

}
