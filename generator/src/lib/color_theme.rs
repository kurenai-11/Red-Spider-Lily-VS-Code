#![allow(non_snake_case)]

use serde::Serialize;

use crate::parsing::EditorColors;

#[derive(Serialize)]
struct UIColors {
    #[serde(rename = "editor.background")]
    editor_background: String,
    #[serde(rename = "editor.foreground")]
    editor_foreground: String,
    #[serde(rename = "editor.foldBackground")]
    editor_foldBackground: String,
    #[serde(rename = "editorLineNumber.foreground")]
    editorLineNumber_foreground: String,
    #[serde(rename = "editorLineNumber.activeForeground")]
    editorLineNumber_activeForeground: String,
    #[serde(rename = "editorCursor.foreground")]
    editor_cursorForeground: String,
    #[serde(rename = "editorLink.activeForeground")]
    editorLink_activeForeground: String,
    #[serde(rename = "textLink.foreground")]
    textLink_foreground: String,
    #[serde(rename = "textLink.activeForeground")]
    textLink_activeForeground: String,
    #[serde(rename = "editorHoverWidget.background")]
    editorHoverWidget_background: String,
    #[serde(rename = "editorHoverWidget.foreground")]
    editorHoverWidget_foreground: String,
    #[serde(rename = "editorHoverWidget.border")]
    editorHoverWidget_border: String,
    #[serde(rename = "activityBar.background")]
    activityBar_background: String,
    #[serde(rename = "activityBar.activeBackground")]
    activityBar_activeBackground: String,
    #[serde(rename = "activityBar.foreground")]
    activityBar_foreground: String,
    #[serde(rename = "activityBar.inactiveForeground")]
    activityBar_inactiveForeground: String,
    #[serde(rename = "activityBarBadge.background")]
    activityBarBadge_background: String,
    #[serde(rename = "statusBar.background")]
    statusBar_background: String,
    #[serde(rename = "sideBar.background")]
    sideBar_background: String,
    #[serde(rename = "sideBar.foreground")]
    sideBar_foreground: String,
    #[serde(rename = "sideBar.border")]
    sideBar_border: String,
    #[serde(rename = "sideBarTitle.foreground")]
    sideBarTitle_foreground: String,
    #[serde(rename = "sideBarSectionHeader.background")]
    sideBarSectionHeader_background: String,
    #[serde(rename = "sideBarSectionHeader.foreground")]
    sideBarSectionHeader_foreground: String,
    #[serde(rename = "sash.hoverBorder")]
    sash_hoverBorder: String,
    #[serde(rename = "widget.shadow")]
    widget_shadow: String,
    #[serde(rename = "selection.background")]
    selection_background: String,
    #[serde(rename = "button.background")]
    button_background: String,
    #[serde(rename = "button.foreground")]
    button_foreground: String,
    #[serde(rename = "icon.foreground")]
    icon_foreground: String,
    #[serde(rename = "focusBorder")]
    focusBorder: String,
    #[serde(rename = "foreground")]
    foreground: String,
    #[serde(rename = "errorForeground")]
    errorForeground: String,
    #[serde(rename = "disabledForeground")]
    disabledForeground: String,
    #[serde(rename = "descriptionForeground")]
    descriptionForeground: String,
    #[serde(rename = "dropdown.background")]
    dropdown_background: String,
    #[serde(rename = "dropdown.border")]
    dropdown_border: String,
    #[serde(rename = "dropdown.listBackground")]
    dropdown_listBackground: String,
    #[serde(rename = "dropdown.foreground")]
    dropdown_foreground: String,
    #[serde(rename = "badge.background")]
    badge_background: String,
    #[serde(rename = "badge.foreground")]
    badge_foreground: String,
    #[serde(rename = "input.background")]
    input_background: String,
    #[serde(rename = "input.foreground")]
    input_foreground: String,
    #[serde(rename = "input.placeholderForeground")]
    input_placeholderForeground: String,
    #[serde(rename = "inputValidation.errorBackground")]
    inputValidation_errorBackground: String,
    #[serde(rename = "inputValidation.errorForeground")]
    inputValidation_errorForeground: String,
    #[serde(rename = "inputValidation.errorBorder")]
    inputValidation_errorBorder: String,
    #[serde(rename = "inputValidation.infoBackground")]
    inputValidation_infoBackground: String,
    #[serde(rename = "inputValidation.infoBorder")]
    inputValidation_infoBorder: String,
    #[serde(rename = "quickInputList.focusBackground")]
    quickInputList_focusBackground: String,
    #[serde(rename = "editorSuggestWidget.background")]
    editorSuggestWidget_background: String,
    #[serde(rename = "quickInput.background")]
    quickInput_background: String,
    #[serde(rename = "peekViewEditor.background")]
    peekViewEditor_background: String,
    #[serde(rename = "peekView.border")]
    peekView_border: String,
    #[serde(rename = "peekViewEditor.matchHighlightBackground")]
    peekViewEditor_matchHighlightBackground: String,
    #[serde(rename = "peekViewResult.background")]
    peekViewResult_background: String,
    #[serde(rename = "peekViewResult.selectionBackground")]
    peekViewResult_selectionBackground: String,
    #[serde(rename = "peekViewTitle.background")]
    peekViewTitle_background: String,
    #[serde(rename = "scrollbar.shadow")]
    scrollbar_shadow: String,
    #[serde(rename = "scrollbarSlider.activeBackground")]
    scrollbarSlider_activeBackground: String,
    #[serde(rename = "scrollbarSlider.background")]
    scrollbarSlider_background: String,
    #[serde(rename = "scrollbarSlider.hoverBackground")]
    scrollbarSlider_hoverBackground: String,
    #[serde(rename = "list.activeSelectionBackground")]
    list_activeSelectionBackground: String,
    #[serde(rename = "list.activeSelectionForeground")]
    list_activeSelectionForeground: String,
    #[serde(rename = "list.activeSelectionIconForeground")]
    list_activeSelectionIconForeground: String,
    #[serde(rename = "list.inactiveSelectionBackground")]
    list_inactiveSelectionBackground: String,
    #[serde(rename = "list.inactiveSelectionIconForeground")]
    list_inactiveSelectionIconForeground: String,
    #[serde(rename = "list.dropBackground")]
    list_dropBackground: String,
    #[serde(rename = "list.hoverBackground")]
    list_hoverBackground: String,
    #[serde(rename = "list.focusHighlightForeground")]
    list_focusHighlightForeground: String,
    #[serde(rename = "list.highlightForeground")]
    list_highlightForeground: String,
    #[serde(rename = "list.errorForeground")]
    list_errorForeground: String,
    #[serde(rename = "list.inactiveFocusBackground")]
    list_inactiveFocusBackground: String,
    #[serde(rename = "list.focusBackground")]
    list_focusBackground: String,
    #[serde(rename = "listFilterWidget.background")]
    listFilterWidget_background: String,
    #[serde(rename = "list.filterMatchBackground")]
    list_filterMatchBackground: String,
    #[serde(rename = "tab.activeBackground")]
    tab_activeBackground: String,
    #[serde(rename = "tab.activeForeground")]
    tab_activeForeground: String,
    #[serde(rename = "tab.inactiveBackground")]
    tab_inactiveBackground: String,
    #[serde(rename = "tab.inactiveForeground")]
    tab_inactiveForeground: String,
    #[serde(rename = "tab.border")]
    tab_border: String,
    #[serde(rename = "tab.hoverBackground")]
    tab_hoverBackground: String,
    #[serde(rename = "editorWidget.background")]
    editorWidget_background: String,
    #[serde(rename = "editorCodeLens.foreground")]
    editorCodeLens_foreground: String,
    #[serde(rename = "panel.background")]
    panel_background: String,
    #[serde(rename = "terminal.background")]
    terminal_background: String,
    #[serde(rename = "terminal.foreground")]
    terminal_foreground: String,
    #[serde(rename = "terminalCursor.foreground")]
    terminalCursor_foreground: String,
    #[serde(rename = "terminal.ansiBrightBlue")]
    terminal_ansiBrightBlue: String,
    #[serde(rename = "toolbar.activeBackground")]
    toolbar_activeBackground: String,
    #[serde(rename = "editorGroupHeader.tabsBackground")]
    editorGroupHeader_tabsBackground: String,
    #[serde(rename = "checkbox.background")]
    checkbox_background: String,
    #[serde(rename = "button.secondaryBackground")]
    button_secondaryBackground: String,

    #[serde(rename = "editor.selectionBackground")]
    editor_selectionBackground: String,
    #[serde(rename = "editor.selectionHighlightBackground")]
    editor_selectionHighlightBackground: String,
    #[serde(rename = "editor.findMatchBackground")]
    editor_findMatchBackground: String,
    #[serde(rename = "editor.findMatchHighlightBackground")]
    editor_findMatchHighlightBackground: String,
    #[serde(rename = "editor.wordHighlightBackground")]
    editor_wordHighlightBackground: String,
    #[serde(rename = "editor.wordHighlightStrongBackground")]
    editor_wordHighlightStrongBackground: String,
    #[serde(rename = "editor.hoverHighlightBackground")]
    editor_hoverHighlightBackground: String,

    #[serde(rename = "symbolIcon.variableForeground")]
    symbolIcon_variableForeground: String,
    #[serde(rename = "symbolIcon.arrayForeground")]
    symbolIcon_arrayForeground: String,
    #[serde(rename = "symbolIcon.booleanForeground")]
    symbolIcon_booleanForeground: String,
    #[serde(rename = "symbolIcon.classForeground")]
    symbolIcon_classForeground: String,
    #[serde(rename = "symbolIcon.colorForeground")]
    symbolIcon_colorForeground: String,
    #[serde(rename = "symbolIcon.constantForeground")]
    symbolIcon_constantForeground: String,
    #[serde(rename = "symbolIcon.constructorForeground")]
    symbolIcon_constructorForeground: String,
    #[serde(rename = "symbolIcon.enumeratorForeground")]
    symbolIcon_enumeratorForeground: String,
    #[serde(rename = "symbolIcon.enumeratorMemberForeground")]
    symbolIcon_enumeratorMemberForeground: String,
    #[serde(rename = "symbolIcon.eventForeground")]
    symbolIcon_eventForeground: String,
    #[serde(rename = "symbolIcon.fieldForeground")]
    symbolIcon_fieldForeground: String,
    #[serde(rename = "symbolIcon.fileForeground")]
    symbolIcon_fileForeground: String,
    #[serde(rename = "symbolIcon.folderForeground")]
    symbolIcon_folderForeground: String,
    #[serde(rename = "symbolIcon.functionForeground")]
    symbolIcon_functionForeground: String,
    #[serde(rename = "symbolIcon.interfaceForeground")]
    symbolIcon_interfaceForeground: String,
    #[serde(rename = "symbolIcon.keyForeground")]
    symbolIcon_keyForeground: String,
    #[serde(rename = "symbolIcon.keywordForeground")]
    symbolIcon_keywordForeground: String,
    #[serde(rename = "symbolIcon.methodForeground")]
    symbolIcon_methodForeground: String,
    #[serde(rename = "symbolIcon.moduleForeground")]
    symbolIcon_moduleForeground: String,
    #[serde(rename = "symbolIcon.namespaceForeground")]
    symbolIcon_namespaceForeground: String,
    #[serde(rename = "symbolIcon.nullForeground")]
    symbolIcon_nullForeground: String,
    #[serde(rename = "symbolIcon.numberForeground")]
    symbolIcon_numberForeground: String,
    #[serde(rename = "symbolIcon.objectForeground")]
    symbolIcon_objectForeground: String,
    #[serde(rename = "symbolIcon.operatorForeground")]
    symbolIcon_operatorForeground: String,
    #[serde(rename = "symbolIcon.packageForeground")]
    symbolIcon_packageForeground: String,
    #[serde(rename = "symbolIcon.propertyForeground")]
    symbolIcon_propertyForeground: String,
    #[serde(rename = "symbolIcon.referenceForeground")]
    symbolIcon_referenceForeground: String,
    #[serde(rename = "symbolIcon.snippetForeground")]
    symbolIcon_snippetForeground: String,
    #[serde(rename = "symbolIcon.stringForeground")]
    symbolIcon_stringForeground: String,
    #[serde(rename = "symbolIcon.structForeground")]
    symbolIcon_structForeground: String,
    #[serde(rename = "symbolIcon.textForeground")]
    symbolIcon_textForeground: String,
    #[serde(rename = "symbolIcon.typeParameterForeground")]
    symbolIcon_typeParameterForeground: String,
    #[serde(rename = "symbolIcon.unitForeground")]
    symbolIcon_unitForeground: String,
}

impl UIColors {
    fn new(main_colors: &EditorColors) -> Self {
        // define some colors and their shades like primary, primary_2
        UIColors {
            // define a color for each field
        }
    }
}

#[derive(Serialize)]
struct TokenStyle {
    fontStyle: String,
    foreground: String,
}

#[derive(Serialize)]
struct TokenColor {
    name: String,
    scope: Vec<String>,
    settings: TokenStyle,
}

#[derive(Serialize)]
pub struct VSCodeColorTheme {
    name: String,
    colors: UIColors,
    tokenColors: Vec<TokenColor>,
}
