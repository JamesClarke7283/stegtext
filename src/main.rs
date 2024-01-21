use fltk::{app, prelude::*, window::Window, button::Button, input::MultilineInput, group::Pack};
use stego_wps::{encode, decode};
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let app = app::App::default();
    let mut win = Window::default().with_size(800, 600).with_label("StegText");
    win.make_resizable(true);

    let cover_input = MultilineInput::new(10, 10, 780, 200, "Cover Text:");
    let secret_input = MultilineInput::new(10, 220, 780, 200, "Secret Text:");

    let toggle_button = Rc::new(RefCell::new(Button::new(10, 430, 200, 40, "Encode")));
    let toggle_button_clone = toggle_button.clone();
    let encode_mode = Rc::new(RefCell::new(true));

    let mut output_display = MultilineInput::new(10, 480, 780, 110, "Output:");
    output_display.set_value("The output will appear here");
    output_display.set_readonly(true);

    let pack = Pack::new(220, 430, 560, 40, "");

    let check_button = Button::new(0, 0, 200, 40, "Check");

    let encode_mode_clone = encode_mode.clone();
    check_button.set_callback(move |_| {
        let cover_text_val = cover_input.value();
        let secret_text_val = secret_input.value();
        if *encode_mode_clone.borrow() {
            match encode(&cover_text_val) {
                Ok(encoded) => {
                    let encoded_str = encoded.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",");
                    output_display.set_value(&encoded_str);
                }
                Err(_) => {
                    output_display.set_value("Error encoding the text.");
                }
            }
        } else {
            let cs = "ABCDEFGHIJKLMNOPQRSTUVWXYZ"; // This will be your character set.
            match decode(&secret_text_val.split_whitespace().filter_map(|s| s.parse().ok()).collect::<Vec<_>>(), cs) {
                Ok(decoded) => {
                    output_display.set_value(&decoded);
                }
                Err(_) => {
                    output_display.set_value("Error decoding the text.");
                }
            }
        }
    });

    toggle_button.borrow_mut().set_callback(move |_| {
        *encode_mode.borrow_mut() = !*encode_mode.borrow();
        let label = if *encode_mode.borrow() { "Encode" } else { "Decode" };
        toggle_button_clone.borrow_mut().set_label(label);
    });

    win.end();
    win.show();
    app.run().unwrap();
}
