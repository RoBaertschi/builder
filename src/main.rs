use druid::{
    widget::{BackgroundBrush, Button, Checkbox, Flex, Label, RadioGroup, TextBox},
    AppLauncher, Color, Data, PlatformError, Widget, WidgetExt, WindowDesc, lens,
};



#[derive(Data, Clone, PartialEq)]
struct ApplicationData {
    checked: bool,
    url: String,
}

// TODO: Dont know what do to with that. Remove if it is not used.

impl Default for ApplicationData {
    fn default() -> Self {
        Self { checked: false, url: String::new() }
    }
}

impl From<bool> for ApplicationData {
    fn from(checked: bool) -> Self {
        let mut returnv = Self::default();
        returnv.checked = checked;
        returnv
    }
}


fn main() -> Result<(), PlatformError> {
    let data: ApplicationData = ApplicationData {
        checked: false,
        url: String::new(),
    };

    AppLauncher::with_window(WindowDesc::new(build_ui()).title("Builder")).launch(data)?;
    Ok(())
}

fn build_ui() -> impl Widget<ApplicationData> {
    let background: BackgroundBrush<ApplicationData> = BackgroundBrush::Color(Color::grey(0.7));
    let radio_group = RadioGroup::column(vec![
        (
            "Build without output.",
            false,
        ),
        (
            "Build with output.",
            true,
        ),
    ]);

    let build_button: Button<ApplicationData> = Button::new("Build");

    let build_button = build_button.on_click(|_event, data, _env| {
        println!(
            "{}",
            data.checked
        )
    });

    let url_textbox: TextBox<String> = TextBox::new();

    let mut label = Label::new("Hello");

    // Set Size
    label.set_text_size(30.0);
    let label = label.padding(5.0);
    // TODO: Get _checkboxes into the Flex layout.
    let _checkboxes: Checkbox = Checkbox::new("Hello World");

    let mut layout = Flex::column();
    layout = layout.with_child(label);
    layout = layout.with_child(radio_group.lens(lens!(ApplicationData, checked)));
    layout = layout.with_child(build_button);
    layout = layout.with_child(url_textbox.lens(lens!(ApplicationData, url)));
    // layout = layout.with_child(checkboxes.into());

    // Last Line: Attention, please put any layout code before this!
    layout.background(background)
}
