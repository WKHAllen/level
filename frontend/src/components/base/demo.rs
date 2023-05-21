use super::*;
use yew::prelude::*;

/// A demo of the base components.
#[function_component]
pub fn Demo() -> Html {
    let input_state = use_state(|| "Input value".to_owned());
    let input_value = (*input_state).clone();
    let textarea_state = use_state(|| "Textarea value".to_owned());
    let textarea_value = (*textarea_state).clone();
    let textarea_state1 = use_state(|| String::new());
    let textarea_state2 = use_state(|| String::new());
    let numberinput_int_state = use_state(|| 3u16);
    let numberinput_int_value = *numberinput_int_state;
    let numberinput_float_state = use_state(|| 1.618f64);
    let numberinput_float_value = *numberinput_float_state;
    let button_state = use_state(|| ButtonStyle::Primary);
    let button_state_primary = button_state.clone();
    let button_state_secondary = button_state.clone();
    let button_state_transparent = button_state.clone();
    let button_state_danger = button_state.clone();
    let button_value = *button_state;
    let checkbox_state = use_state(|| true);
    let checkbox_value = *checkbox_state;
    let switch_state = use_state(|| true);
    let switch_value = *switch_state;
    let radio_state = use_state(|| None);
    let radio_value = *radio_state;
    let slider_int_state = use_state(|| 3u8);
    let slider_int_value = *slider_int_state;
    let slider_float_state = use_state(|| 1.6f32);
    let slider_float_value = *slider_float_state;
    let icon_button_state = use_state(|| 0usize);
    let icon_button_value = *icon_button_state;
    let select_state = use_state(|| 0);
    let select_value = *select_state;

    html! {
        <div class="base-demo">
            <div class="base-demo-item">
                <span class="base-demo-item-label">{"Error"}</span>
                <Error message="A large error message" size={ErrorSize::Larger} />
            </div>
            <div class="base-demo-item">
                <span class="base-demo-item-label">{"Input"}</span>
                <Input state={input_state.clone()} label="Input label" placeholder="Placeholder!" required={true} error={input_value.is_empty().then_some("Please enter a value")} />
                <span>{"Value: "}{input_value}</span>
                <Input state={input_state} label="Disabled input" disabled={true} />
            </div>
            <div class="base-demo-item">
                <span class="base-demo-item-label">{"Textarea"}</span>
                <TextArea state={textarea_state.clone()} label="Textarea label" placeholder="Placeholder!" required={true} error={textarea_value.is_empty().then_some("Please enter a value")} />
                <span>{"Value: "}{textarea_value}</span>
                <TextArea state={textarea_state} label="Disabled textarea" disabled={true} resize={TextAreaResize::Horizontal} />
                <TextArea state={textarea_state1} label="Vertical resize" resize={TextAreaResize::Vertical} />
                <TextArea state={textarea_state2} label="Full resize" resize={TextAreaResize::Both} />
            </div>
            <div class="base-demo-item">
                <span class="base-demo-item-label">{"Number input"}</span>
                <NumberInput<u16> state={numberinput_int_state.clone()} label="Int number input label" placeholder="Placeholder!" min={0} max={100} required={true} error={(numberinput_int_value == 3).then_some("How about something other than 3")} />
                <span>{"Value: "}{numberinput_int_value}</span>
                <NumberInput<f64> state={numberinput_float_state} label="Float number input label" placeholder="Placeholder!" min={-5.0} max={5.0} decimals={5} required={true} error={(numberinput_float_value == 3.14).then_some("No pi, please")} />
                <span>{"Value: "}{numberinput_float_value}</span>
                <NumberInput<u16> state={numberinput_int_state} label="Disabled number input" disabled={true} />
            </div>
            <div class="base-demo-item">
                <span class="base-demo-item-label">{"Button"}</span>
                <Button text="Primary" on_click={move |_| button_state_primary.set(ButtonStyle::Primary)} />
                <Button text="Secondary" style={ButtonStyle::Secondary} on_click={move |_| button_state_secondary.set(ButtonStyle::Secondary)} />
                <Button text="Transparent" style={ButtonStyle::Transparent} on_click={move |_| button_state_transparent.set(ButtonStyle::Transparent)} />
                <Button text="Danger" style={ButtonStyle::Danger} on_click={move |_| button_state_danger.set(ButtonStyle::Danger)} />
                <Button text="Disabled" style={*button_state} disabled={true} />
                <span>{"Last clicked: "}{button_value.style_name()}</span>
            </div>
            <div class="base-demo-item">
                <span class="base-demo-item-label">{"Checkbox"}</span>
                <Checkbox state={checkbox_state.clone()} label="Checkbox label" />
                <span>{"Value: "}{checkbox_value.to_string()}</span>
                <Checkbox state={checkbox_state} label="Disabled checkbox" disabled={true} />
            </div>
            <div class="base-demo-item">
                <span class="base-demo-item-label">{"Switch"}</span>
                <Switch state={switch_state.clone()} label="Switch label" />
                <span>{"Value: "}{switch_value.to_string()}</span>
                <Switch state={switch_state} label="Disabled switch" disabled={true} />
            </div>
            <div class="base-demo-item">
                <span class="base-demo-item-label">{"Radio group"}</span>
                <RadioGroup state={radio_state.clone()}>
                    <RadioButton>{"Option 1"}</RadioButton>
                    <RadioButton>{"Option 2"}</RadioButton>
                    <RadioButton>{"Option 3"}</RadioButton>
                    <RadioButton disabled={true}>{"Option 4"}</RadioButton>
                </RadioGroup>
                <span>{"Value: "}{radio_value.map(|x| x.to_string()).unwrap_or("None".to_owned())}</span>
                <RadioGroup state={radio_state} orientation={RadioGroupOrientation::Horizontal} disabled={true}>
                    <RadioButton>{"Option 1"}</RadioButton>
                    <RadioButton>{"Option 2"}</RadioButton>
                    <RadioButton>{"Option 3"}</RadioButton>
                </RadioGroup>
            </div>
            <div class="base-demo-item">
                <span class="base-demo-item-label">{"Slider"}</span>
                <Slider<u8> state={slider_int_state.clone()} min={1} max={9} step={2} label="Int slider label" />
                <span>{"Value: "}{slider_int_value.to_string()}</span>
                <Slider<f32> state={slider_float_state.clone()} min={-10.0} max={10.0} step={0.1} label="Float slider label" />
                <span>{"Value: "}{slider_float_value.to_string()}</span>
                <Slider<u8> state={slider_int_state} min={1} max={17} label="Disabled slider" disabled={true} />
            </div>
            <div class="base-demo-item">
                <span class="base-demo-item-label">{"Icon"}</span>
                <Icon name="angle-down-solid" />
                <Icon name="angle-down-solid" disabled={true} />
            </div>
            <div class="base-demo-item">
                <span class="base-demo-item-label">{"Icon button"}</span>
                <IconButton name="angle-down-solid" on_click={move |_| icon_button_state.set(icon_button_value + 1)} />
                <span>{"Icon button has been clicked "}{icon_button_value}{" times"}</span>
                <IconButton name="angle-down-solid" disabled={true} />
            </div>
            <div class="base-demo-item">
                <span class="base-demo-item-label">{"Select"}</span>
                <Select state={select_state.clone()} label="Select label" error={(select_value == 3).then_some("This option isn't available for the disabled select box below")}>
                    <SelectOption>{"Option 1"}</SelectOption>
                    <SelectOption>{"Option 2"}</SelectOption>
                    <SelectOption>{"Option 3"}</SelectOption>
                    <SelectOption>{"Option 4"}</SelectOption>
                    <SelectOption disabled={true}>{"Option 5 (disabled)"}</SelectOption>
                    <SelectOption disabled={true}>{"Option 6 (disabled)"}</SelectOption>
                    <SelectOption disabled={true}>{"Option 7 (disabled)"}</SelectOption>
                    <SelectOption disabled={true}>{"Option 8 (disabled)"}</SelectOption>
                    <SelectOption disabled={true}>{"Option 9 (disabled)"}</SelectOption>
                    <SelectOption disabled={true}>{"Option 10 (disabled)"}</SelectOption>
                    <SelectOption disabled={true}>{"Option 11 (disabled)"}</SelectOption>
                    <SelectOption disabled={true}>{"Option 12 (disabled)"}</SelectOption>
                    <SelectOption disabled={true}>{"Option 13 (disabled)"}</SelectOption>
                    <SelectOption disabled={true}>{"Option 14 (disabled)"}</SelectOption>
                    <SelectOption disabled={true}>{"Option 15 (disabled)"}</SelectOption>
                </Select>
                <span>{"Value: "}{select_value.to_string()}</span>
                <Select state={select_state} label="Disabled select label" disabled={true}>
                    <SelectOption>{"Option 1"}</SelectOption>
                    <SelectOption>{"Option 2"}</SelectOption>
                    <SelectOption>{"Option 3"}</SelectOption>
                </Select>
            </div>
        </div>
    }
}
