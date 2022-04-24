use gio::SimpleAction;
use glib::clone;
use gtk::{gio, glib};
use gtk::{prelude::*, Align};
use gtk::{Application,ApplicationWindow, Button, Label, Orientation};

use std::env;

use crate::config::{APP_ID,GETTEXT_PACKAGE};
use crate::ihm::ihm_utilits;

pub fn ui() {
    // Create a new application
    let app = Application::new(Some(APP_ID), Default::default());

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    let mut args: Vec<String> = env::args().collect();
    let size = args.len();
    for i in 1..size {
        args.swap_remove(i as usize);
    }

    // Run the application
    app.run_with_args(&args);
}

fn build_ui(app: &Application) {
    let original_state = 0;
    let label = Label::builder()
        .label(&format!("Counter: {}", original_state))
        .build();
    // Create a button with label and action
    let button_incr = Button::builder()
        .label("+")
        .action_name("win.count")
        .action_target(&1.to_variant())
        .build();
    let button_decr = Button::builder()
        .label("-")
        .action_name("win.count")
        .action_target(&2.to_variant())
        .build();

    let gtk_box_sub = gtk::Box::builder()
        .orientation(Orientation::Horizontal)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .spacing(12)
        .halign(Align::Center)
        .build();

    gtk_box_sub.append(&button_incr);
    gtk_box_sub.append(&button_decr);

    // Create `gtk_box` and add `button` and `label` to it
    let gtk_box = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .spacing(12)
        .halign(Align::Center)
        .build();
    gtk_box.append(&gtk_box_sub);
    gtk_box.append(&label);

    // Create a window and set the title
    let window = ApplicationWindow::builder()
        .application(app)
        .title(GETTEXT_PACKAGE)
        .width_request(360)
        .child(&gtk_box)
        .build();

    // Add action "count" to `window` taking an integer as parameter
    let action_count = SimpleAction::new_stateful(
        "count",
        Some(&i32::static_variant_type()),
        &original_state.to_variant(),
    );

    action_count.connect_activate(clone!(@weak label => move |action, parameter| {
        // Get state
        let old_state = action
        .state()
        .expect("Could not get state.")
        .get::<i32>()
        .expect("The value needs to be of type `i32`.");

        let mut state = old_state;

        // Get parameter
        let parameter = parameter
            .expect("Could not get parameter.")
            .get::<i32>()
            .expect("The value needs to be of type `i32`.");

        if parameter == 1 {
            state += 1;
        } else {
            state -= 1;
        }

        ihm_utilits::info_col(1, &format!("value {} --> {} opt: {}", old_state, state, parameter));

        action.set_state(&state.to_variant());

        // Update label with new state
        label.set_label(&format!("Counter: {}", state));
    }));

    window.add_action(&action_count);

    // Present window
    window.present();
}
