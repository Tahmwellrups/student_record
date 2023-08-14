use druid::{LocalizedString, Data, Lens, widget::{Label, Button, Flex}, Env, Widget, WindowDesc,
            Color, EventCtx, Event, Selector, Target, AppLauncher, WidgetExt};
use druid::widget::{Align, TextBox};


#[derive(Clone, Data, Lens)]
struct UIData {
    name: String,
    q1: String,
    q2: String,
    q3: String
}

fn input_section() -> impl Widget<UIData> {
    let name_label = Label::new(format!("Name: "));
    let q1_label = Label::new(format!("Quiz 1: "));
    let q2_label = Label::new(format!("Quiz 2: "));
    let q3_label = Label::new(format!("Quiz 3: "));

    let name_txt = TextBox::new().fix_width(200.0).lens(UIData::name);
    let q1_txt = TextBox::new().fix_width(200.0).lens(UIData::q1);
    let q2_txt = TextBox::new().fix_width(200.0).lens(UIData::q2);
    let q3_txt = TextBox::new().fix_width(200.0).lens(UIData::q3);

    let button = Button::from_label(Label::new("Register").with_text_color(Color::grey(0.5)))
        .on_click(|_ctx, data: &mut UIData, _env| {
            // Update the UIData with the new values.
            let new_name = data.name.clone();
            let new_q1 = data.q1.clone();
            let new_q2 = data.q2.clone();
            let new_q3 = data.q3.clone();

            // Update the UIData with the new values.
            data.name = new_name;
            data.q1 = new_q1;
            data.q2 = new_q2;
            data.q3 = new_q3;
        });

    let input = Flex::column()
        .with_spacer(50.0)
        .with_child(Flex::row().with_child(name_label).with_child(name_txt))
        .with_spacer(20.0)
        .with_child(Flex::row().with_child(q1_label).with_child(q1_txt))
        .with_spacer(20.0)
        .with_child(Flex::row().with_child(q2_label).with_child(q2_txt))
        .with_spacer(20.0)
        .with_child(Flex::row().with_child(q3_label).with_child(q3_txt))
        .with_spacer(20.0)
        .with_child(button);


    Align::centered(input)
}

fn display_section() -> impl Widget<UIData> {
    let name_dis = Label::new(|data: &UIData, _env: &Env| format!("Name: {}", data.name));
    let q1_dis = Label::new(|data: &UIData, _env: &Env| format!("Quiz 1: {}", data.q1));
    let q2_dis = Label::new(|data: &UIData, _env: &Env| format!("Quiz 2: {}", data.q2));
    let q3_dis = Label::new(|data: &UIData, _env: &Env| format!("Quiz 3: {}", data.q3));

    let display = Flex::column()
        .with_child(name_dis)
        .with_child(q1_dis)
        .with_child(q2_dis)
        .with_child(q3_dis);

    Align::centered(display)
}

pub(crate) fn main_ui() {
    let main_window = WindowDesc::new(ui_builder());

    let new_data = UIData {
        name: String::new(),
        q1: String::new(),
        q2: String::new(),
        q3: String::new()
    };

    AppLauncher::with_window(main_window)
        .use_simple_logger().launch(new_data).unwrap();

}

fn ui_builder() -> impl Widget<UIData> {
    let main_content = Flex::row()
        .with_flex_child(input_section(), 1.0)
        .with_flex_child(display_section(), 1.0);

    main_content
}