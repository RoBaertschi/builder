use druid::{PlatformError, Widget, widget::{
    Flex, RadioGroup, BackgroundBrush, Checkbox
}, AppLauncher, WindowDesc, Data, Color, WidgetExt};

#[derive(Data, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct RadioData {
    checked: bool
}

#[derive(Data, Clone, PartialEq)]
enum Checked {
    True,
    False
}

// TODO: Dont know what do to with that. Remove if it is not used.

/*impl From<bool> for Checked {
    fn from(this: bool) -> Self {
        match this {
            true => Self::True,
            false => Self::False,
        }
    }
}

impl From<Checked> for bool {
    fn from(this: Checked) -> Self {
        match this {
            True => true,
            False => false,
        }
    }
}*/

fn main() -> Result<(), PlatformError> {

    let _data: RadioData = RadioData {
        checked: false
    };

    let checked = Checked::True;

    AppLauncher::with_window(WindowDesc::new(build_ui()).title("Hello World"))
        .launch(checked)?;
        Ok(())
    }


fn build_ui() -> impl Widget<Checked> {
    // shitty: Do Not USE! let data = RadioData {checked: false};
    
    let background: BackgroundBrush<Checked> = BackgroundBrush::Color(Color::grey(0.7));
    let radio_group = RadioGroup::column(vec!(("Hello World", Checked::False), ("Hello World2", Checked::True)));

    // TODO: Get _checkboxes into the Flex layout.
    let _checkboxes: Checkbox = Checkbox::new("Hello World");

    Flex::column()
        .with_child(radio_group)
        .background(background)
}