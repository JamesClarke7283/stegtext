use fltk::{
    app, app::Sender, button::Button, enums::Shortcut, frame::Frame, input::MultilineInput,
    menu::MenuBar, menu::MenuFlag, prelude::*, window::Window,
};
use std::cell::RefCell;
use std::rc::Rc;
use stego_wps::{compare, decode, encode};

fn main() {
    let app = app::App::default();
    let mut win = Window::default().with_size(800, 600).with_label("StegText");
    win.make_resizable(true);

    let mut menu_bar = MenuBar::new(0, 0, 800, 30, "");
    let (sender, receiver) = app::channel::<()>();

    menu_bar.add_emit("&File/Exit", Shortcut::None, MenuFlag::Normal, sender, ());
    /* To be implemented later.
    menu_bar.add_choice("&Help/User Guide", 0, move |_| {
        // Logic to show User Guide window
    });
    menu_bar.add_choice("&Help/Troubleshooting", 0, move |_| {
        // Logic to show Troubleshooting window
    });
    */

    let cover_input = MultilineInput::new(10, 10, 780, 200, "Cover Text:");
    let secret_input = MultilineInput::new(10, 220, 780, 200, "Secret Text:");
    let compare_output = Rc::new(RefCell::new(Frame::new(10, 550, 780, 40, "")));
    let toggle_button = Rc::new(RefCell::new(Button::new(10, 430, 200, 40, "Encode On")));
    let output_display = Rc::new(RefCell::new(MultilineInput::new(
        10, 480, 780, 60, "Output:",
    )));
    output_display
        .borrow_mut()
        .set_value("The output will appear here");
    output_display.borrow_mut().set_readonly(true);

    let encode_mode = Rc::new(RefCell::new(true));
    let toggle_button_clone = toggle_button.clone();

    toggle_button.borrow_mut().set_callback({
        let encode_mode_clone = encode_mode.clone();
        let cover_input_clone = cover_input.clone();
        let secret_input_clone = secret_input.clone();
        let output_display = output_display.clone();
        let compare_output = compare_output.clone();

        move |_| {
            let mode = *encode_mode_clone.borrow();
            let label = if mode { "Encode On" } else { "Decode On" };
            toggle_button_clone.borrow_mut().set_label(label);

            // Clone the Rc objects inside the closure
            let output_display_clone = output_display.clone();
            let compare_output_clone = compare_output.clone();

            // Call the update_output function
            update_output(
                &cover_input_clone,
                &secret_input_clone,
                output_display_clone,
                compare_output_clone,
                mode,
            );
        }
    });

    win.end();
    win.show();
    while app.wait() {
        if let Some(msg) = receiver.recv() {
            // Do something with msg, like quit the app
            app.quit();
        }
    }
    app.run().unwrap();
}

fn update_output(
    cover_input: &MultilineInput,
    secret_input: &MultilineInput,
    output_display: Rc<RefCell<MultilineInput>>,
    compare_output: Rc<RefCell<Frame>>,
    encode_mode: bool,
) {
    let cover_text_val = cover_input.value();
    let secret_text_val = secret_input.value();
    let cs = "ABCDEFGHIJKLMNOPQRSTUVWXYZ"; // This will be your character set.

    if encode_mode {
        // Encode mode
        match encode(&cover_text_val) {
            Ok(encoded) => {
                let encoded_str = encoded
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(",");
                output_display.borrow_mut().set_value(&encoded_str);
            }
            Err(_) => {
                output_display
                    .borrow_mut()
                    .set_value("Error encoding the text.");
            }
        }
    } else {
        // Decode mode
        match decode(
            &secret_text_val
                .split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect::<Vec<_>>(),
            cs,
        ) {
            Ok(decoded) => {
                output_display.borrow_mut().set_value(&decoded);
            }
            Err(_) => {
                output_display
                    .borrow_mut()
                    .set_value("Error decoding the text.");
            }
        }
    }

    // Update compare output
    match compare(&secret_text_val, &cover_text_val, cs) {
        Ok(changes) => {
            let changes_str = changes
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(", ");
            compare_output
                .borrow_mut()
                .set_label(&format!("Changes: {}", changes_str));
        }
        Err(e) => {
            compare_output
                .borrow_mut()
                .set_label(&format!("Comparison error: {}", e));
        }
    }
}
