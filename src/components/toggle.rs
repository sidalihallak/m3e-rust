use super::action_control::ActionControl;
use super::{
    ButtonGroupSelection, ButtonShape, ButtonSize, ButtonVariant, ConnectedButtonGroup,
    ConnectedButtonItem, ConnectedButtonVariant, IconData, StandardButtonGroup, StandardButtonItem,
    StandardGroupSelection,
};
use dioxus::prelude::*;
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum ToggleVariant {
    #[default]
    Default,
    Filled,
    Tonal,
    Outlined,
    Elevated,
}
impl ToggleVariant {
    fn button(self) -> ButtonVariant {
        match self {
            Self::Default => ButtonVariant::Text,
            Self::Filled => ButtonVariant::Filled,
            Self::Tonal => ButtonVariant::Tonal,
            Self::Outlined => ButtonVariant::Outlined,
            Self::Elevated => ButtonVariant::Elevated,
        }
    }
}
fn selected_glyph(icon: Option<IconData>) -> Option<IconData> {
    use crate::icons::*;
    icon.map(|icon| {
        [
            (FAVORITE, FAVORITE_FILL),
            (HOME, HOME_FILL),
            (PALETTE, PALETTE_FILL),
            (TOUCH_APP, TOUCH_APP_FILL),
            (TUNE, TUNE_FILL),
            (CHAT_BUBBLE, CHAT_BUBBLE_FILL),
            (LAYERS, LAYERS_FILL),
            (EXPLORE, EXPLORE_FILL),
        ]
        .into_iter()
        .find_map(|(outline, filled)| (icon == outline).then_some(filled))
        .unwrap_or(icon)
    })
}
/// Controlled native toggle, sharing the accepted press lifecycle.
/// Built-in paired glyphs fill when selected; supply selected_icon for custom glyphs.
#[component]
pub fn Toggle(
    label: String,
    #[props(default)] selected: bool,
    #[props(default)] icon: Option<IconData>,
    #[props(default)] selected_icon: Option<IconData>,
    #[props(default)] selected_label: Option<String>,
    #[props(default)] icon_only: bool,
    #[props(default)] disabled: bool,
    #[props(default)] variant: ToggleVariant,
    #[props(default)] size: ButtonSize,
    #[props(default)] shape: ButtonShape,
    #[props(default)] class: String,
    #[props(default)] onchange: EventHandler<bool>,
) -> Element {
    let icon = if selected {
        selected_icon.or_else(|| selected_glyph(icon))
    } else {
        icon
    };
    let label = if selected {
        selected_label.unwrap_or(label)
    } else {
        label
    };
    let class = format!(
        "m3-toggle {} {class}",
        if variant == ToggleVariant::Default {
            "m3-toggle--default"
        } else {
            ""
        }
    );
    rsx! {ActionControl{label,icon,icon_only,pressed:Some(selected),disabled,variant:variant.button(),size,shape,class,onclick:move|_|{if !disabled{onchange.call(!selected);}}}}
}
#[derive(Clone, Debug, PartialEq)]
pub struct ToggleGroupItem {
    pub value: String,
    pub label: String,
    pub icon: Option<IconData>,
    pub selected_icon: Option<IconData>,
    pub icon_only: bool,
    pub disabled: bool,
}
impl ToggleGroupItem {
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            icon: None,
            selected_icon: None,
            icon_only: false,
            disabled: false,
        }
    }
}
/// Horizontal group with stable string values. Connected uses 2px gaps;
/// standard uses Material per-size gaps and neighbour width response.
/// `selection_required` expects an initial nonempty valid selection.
#[component]
pub fn ToggleGroup(
    label: String,
    items: Vec<ToggleGroupItem>,
    #[props(default)] selected: Vec<String>,
    #[props(default)] multiple: bool,
    #[props(default)] connected: bool,
    #[props(default)] selection_required: bool,
    #[props(default)] disabled: bool,
    #[props(default)] variant: ToggleVariant,
    #[props(default)] size: ButtonSize,
    #[props(default)] shape: ButtonShape,
    #[props(default)] class: String,
    #[props(default)] onchange: EventHandler<Vec<String>>,
) -> Element {
    let items = items
        .into_iter()
        .map(|mut item| {
            if selected.contains(&item.value) {
                item.icon = item.selected_icon.or_else(|| selected_glyph(item.icon));
            }
            item
        })
        .collect::<Vec<_>>();
    let indices = items
        .iter()
        .enumerate()
        .filter_map(|(i, item)| selected.contains(&item.value).then_some(i))
        .collect::<Vec<_>>();
    let values = items.iter().map(|i| i.value.clone()).collect::<Vec<_>>();
    let update = move |next: Vec<usize>| {
        onchange.call(
            next.into_iter()
                .filter_map(|i| values.get(i).cloned())
                .collect(),
        )
    };
    if connected {
        let variant = match variant {
            ToggleVariant::Default | ToggleVariant::Filled => ConnectedButtonVariant::Filled,
            ToggleVariant::Tonal => ConnectedButtonVariant::Tonal,
            ToggleVariant::Outlined => ConnectedButtonVariant::Outlined,
            ToggleVariant::Elevated => ConnectedButtonVariant::Elevated,
        };
        let items = items
            .into_iter()
            .map(|i| ConnectedButtonItem {
                label: i.label,
                icon: i.icon,
                icon_only: i.icon_only,
                disabled: i.disabled,
            })
            .collect();
        rsx! {ConnectedButtonGroup{items,selected:indices,selection:if multiple{ButtonGroupSelection::Multiple}else{ButtonGroupSelection::Single},selection_required,disabled,variant,size,shape,aria_label:Some(label),class,onchange:update}}
    } else {
        let class = format!(
            "m3-toggle-group {} {class}",
            if variant == ToggleVariant::Default {
                "m3-toggle-group--default"
            } else {
                ""
            }
        );
        let items = items
            .into_iter()
            .map(|i| StandardButtonItem {
                label: i.label,
                icon: i.icon,
                icon_only: i.icon_only,
                disabled: i.disabled,
                variant: variant.button(),
            })
            .collect();
        rsx! {StandardButtonGroup{items,selected:indices,selection:if multiple{StandardGroupSelection::Multiple}else{StandardGroupSelection::Single},selection_required,disabled,size,shape,aria_label:Some(label),class,onchange:update}}
    }
}
