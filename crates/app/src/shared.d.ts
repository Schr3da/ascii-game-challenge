/* tslint:disable */
/* eslint-disable */
export interface UiLabel {
    id: ViewComponentIds;
    text: string;
    alignment: TextAlignment;
    shortcut: string | null;
}

export type TextAlignment = "Center" | "Left" | "Right";

export type ViewComponentIds = { Main: MainMenuIds } | { Options: OptionMenuIds } | { Game: GameIds } | { Popup: PopupIds };

export type GameIds = "Day" | "Time" | "Turns" | "Canvas" | "Menu" | "Actions" | "None";

export interface SelectedCell {
    top: number;
    bottom: number;
    frame: Rect;
    cell: Cell;
}

export interface Cell {
    id: AsciiIds;
    symbol: Ascii;
    background: CellColors;
    foreground: CellColors;
    isBold: boolean;
    shortDescription: string | null;
    longDescription: string | null;
}

export interface Asset {
    id: AssetTypes;
    cells: Cell[];
    description: string | null;
}

export type AssetTypes = "wall" | "unknownAssetType";

export type Ascii = "space" | "plus" | "tilde" | "doubleTilde" | "hash" | "h" | "u";

export interface PopupState {
    currentSelectedGameTile: SelectedCell;
}

export interface GameViewFooterState {}

export interface GameViewHeaderState {
    currentDays: string;
    currentHours: string;
    currentMinutes: string;
    tickCount: string;
}

export type EcsSubscriptionIds = "GeneralSubscription" | "ViewSubscription" | "PopupSubscription" | "GameMetaSubscription";

export type LayoutConstraints = { Percentage: number } | { Value: number } | { MinValue: number } | { MaxValue: number };

export type LayoutAlignments = "Horizontal" | "Vertical";

export type UiViewIds = "Main" | "Game" | "Options" | { Popup: UiPopupViewIds } | "Quit";

export interface UiList {
    id: ViewComponentIds;
    label: string;
    children: UiLabel[];
}

export interface Position {
    x: number;
    y: number;
}

export type InputEvents = { Keyboard: KeyboardEvent } | { Mouse: MouseEvent };

export type KeyboardEvent = { OnPress: Keys };

export type MouseEvent = { OnPress: boolean } | { OnMove: [number, number] };

export type Keys = "UpArrow" | "DownArrow" | "LeftArrow" | "RightArrow" | "Enter" | "Esc" | "Backspace" | "Tab" | "BackTab" | { Char: string };

export type PopupIds = { Log: LoggerIds | null } | { Build: BuildingIds | null } | "UnknownCommandId";

export type ViewDataTypes = "NoViewData" | { GameHeader: GameViewHeaderState } | { Popup: PopupState } | { Logger: LoggerState };

export type SelectionDirections = "Next" | "Previous";

export type Navigation = "Up" | "Down" | "Left" | "Right" | { Custom: [number, number] };

export interface GameMeta {
    status: GameStatus;
    cursor: SelectedCell | null;
}

export type GameStatus = "GameDidNotStart" | "GameDidStart" | "GameDidPaused" | "GameWillEnded";

export type LoggerIds = "Print";

export type BuildingIds = "Lumbarjack";

export interface UiView {
    id: UiViewIds;
    layout: UiLayout;
    state: UiViewState;
    children: UiViewChild[];
}

export type UiViewChild = { List: UiList } | { Label: UiLabel } | { Section: UiView } | "Placeholder" | { GameCanvas: [Cell, Position][] };

export interface UiViewState {
    selected_id: ViewComponentIds;
    selectable_ids: ViewComponentIds[];
    view_data: ViewDataTypes;
}

export interface UiLayout {
    margin: number;
    alignment: LayoutAlignments;
    constraints: LayoutConstraints[];
}

export interface Rect {
    x: number;
    y: number;
    width: number;
    height: number;
}

export type CellColors = { rgb: [number, number, number] };

export type AsciiIds = "notVisible" | "visible" | "sand" | "shallowWater" | "deepWater" | "headQuarter" | "unknownAsciiId";

export type WindowEvents = { Resize: [number, number] };

export type EcsEvents = { Send: SendEvents } | { Subscriber: SubscriptionEvents };

export type SubscriptionEvents = { General: GeneralSubscription } | { Ui: UiSubscription } | { Renderer: RenderSubscription };

export type RenderSubscription = { OnWorldDidUpdate: [UiView | null, UiView | null, GameMeta] };

export type UiSubscription = "UnknownUiSubscription";

export type GeneralSubscription = "OnApplicationDidStart" | { OnApplicationDidLoadAssets: Record<AsciiIds, Cell> } | "OnApplicationDidInitialise" | "OnApplicationDidClose";

export type SendEvents = { General: GeneralEvents } | { Ui: UiEvents } | { Renderer: RenderEvents } | "Tick";

export type RenderEvents = { OnUpdateCamera: Navigation } | { OnUpdateSelectedCell: Navigation } | "OnWorldWillUpdate";

export type UiEvents = { OnSelect: SelectionDirections } | { OnSelectById: ViewComponentIds } | { OnClick: ViewComponentIds } | { OnShortcut: string } | { OnOpenPopup: UiPopupViewIds } | "OnClosePopup" | "OnCloseView";

export type GeneralEvents = { OnApplicationResize: [number, number] } | { OnApplicationWillInitialise: [number, number] } | "OnApplicationWillClose";

export interface LoggerState {
    currentLogs: string[];
}

export type UiPopupViewIds = "Actions" | "Logs" | "Buildings";

export type OptionMenuIds = "Title" | "OptionList" | "LevelOfDifficulty" | "Sound" | "Back";

export type MainMenuIds = "Title" | "MenuList" | "NewGame" | "Options" | "Quit";

