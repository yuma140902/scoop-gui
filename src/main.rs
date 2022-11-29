use native_windows_derive::NwgUi;
use native_windows_gui as nwg;
use nwg::NativeUi;

#[derive(Default, NwgUi)]
pub struct App {
    #[nwg_control(size: (300, 135), title: "Scoop GUI", flags: "WINDOW|VISIBLE")]
    #[nwg_events(OnWindowClose: [App::say_goodbye])]
    window: nwg::Window,

    #[nwg_control(text: "yuma", size: (280, 35), position: (10, 10), focus: true)]
    name_box: nwg::TextInput,

    #[nwg_control(text: "Say my name", size: (280, 70), position: (10, 50))]
    #[nwg_events(OnButtonClick: [App::say_hello])]
    hello_button: nwg::Button,
}

impl App {
    fn say_hello(&self) {
        nwg::modal_info_message(
            &self.window,
            "Hello",
            &format!("Hello {}", self.name_box.text()),
        );
    }

    fn say_goodbye(&self) {
        nwg::modal_info_message(
            &self.window,
            "Goodbye",
            &format!("Goodbye {}", self.name_box.text()),
        );
        nwg::stop_thread_dispatch();
    }
}

fn main() {
    nwg::init().expect("Failed to initialize Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set font to Segoe UI");
    let _app = App::build_ui(Default::default()).expect("Failed to build UI");
    nwg::dispatch_thread_events();
}
