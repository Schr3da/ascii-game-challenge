/* tslint:disable */
/* eslint-disable */
export type OptionMenuIds = "Title" | "OptionList" | "LevelOfDifficulty" | "Sound" | "Back";

export type GameIds = "Day" | "Time" | "Turns" | "Canvas" | "Menu" | "Actions" | "None";

export type CommandIds = "Move" | { Build: BuildingIds | null } | "Inspect" | "UnknownCommandId";

export type UiViewIds = "Main" | "Game" | "Options" | { Popup: UiPopupViewIds } | "Quit";

export type BuildingIds = "Lumbarjack";

export type ViewDataTypes = "NoViewData" | { GameHeader: GameViewHeaderState } | { Popup: PopupState };

export interface PopupState {
    currentSelectedGameTile: SelectedCell;
}

export interface Asset {
    id: AssetTypes;
    cells: Cell[];
    description: string | null;
}

export type AssetTypes = "wall" | "unknownAssetType";

export type EcsEvents = { Send: SendEvents } | { Subscriber: SubscriptionEvents };

export type SubscriptionEvents = { General: GeneralSubscription } | { Ui: UiSubscription } | { Renderer: RenderSubscription } | { Command: CommandSubscription };

export type SendEvents = { General: GeneralEvents } | { Ui: UiEvents } | { Commands: CommandInputEvents } | { Renderer: RenderEvents } | "Tick";

export interface UiList {
    id: ViewComponentIds;
    label: string;
    children: UiLabel[];
}

export interface UiLayout {
    margin: number;
    alignment: LayoutAlignments;
    constraints: LayoutConstraints[];
}

export interface SelectedCell {
    top: number;
    bottom: number;
    frame: Rect;
    cell: Cell;
}

export interface Rect {
    x: number;
    y: number;
    width: number;
    height: number;
}

export interface Position {
    x: number;
    y: number;
}

export type LayoutConstraints = { Percentage: number } | { Value: number } | { MinValue: number } | { MaxValue: number };

export type LayoutAlignments = "Horizontal" | "Vertical";

export interface GameMeta {
    status: GameStatus;
    cursor: SelectedCell | null;
}

export type MainMenuIds = "Title" | "MenuList" | "NewGame" | "Options" | "Quit";

export type ViewComponentIds = { Main: MainMenuIds } | { Options: OptionMenuIds } | { Game: GameIds } | { CommandPopup: CommandIds };

export interface UiLabel {
    id: ViewComponentIds;
    text: string;
    alignment: TextAlignment;
    shortcut: string | null;
}

export interface GameViewFooterState {}

export interface GameViewHeaderState {
    currentDays: string;
    currentHours: string;
    currentMinutes: string;
    tickCount: string;
}

export interface Interaction {
    is_enabled: boolean;
    is_selected: boolean;
}

export type SelectedCellNavigation = "Up" | "Down" | "Left" | "Right" | { Custom: [number, number] };

export type SelectionDirections = "Next" | "Previous";

export interface UiView {
    id: UiViewIds;
    layout: UiLayout;
    state: UiViewState;
    children: UiViewChild[];
}

export type UiViewChild = { List: UiList } | { Label: UiLabel } | { Section: UiView } | "Placeholder" | { GameCanvas: [Cell, Position][] };

export type GeneralSubscription = "OnApplicationDidStart" | { OnApplicationDidLoadAssets: Record<AsciiIds, Cell> } | "OnApplicationDidInitialise" | "OnApplicationDidClose";

export type GeneralEvents = { OnApplicationResize: [number, number] } | { OnApplicationWillInitialise: [number, number] } | "OnApplicationWillClose";

export type CommandSubscription = { OnCommandDidUpdate: string[] };

export type CommandInputEvents = "New" | "Pop" | { Push: string } | { Execute: string[] } | "Cancel";

export type UiPopupViewIds = "Command" | "Buildings";

export interface Sprite {
    asset: Asset;
    frame: Rect;
}

export interface Cell {
    id: AsciiIds;
    symbol: Ascii;
    background: CellColors;
    foreground: CellColors;
    isBold: boolean;
}

export type RenderSubscription = { OnWorldDidUpdate: [UiView | null, UiView | null, GameMeta] };

export type RenderEvents = { OnUpdateCamera: CameraNavigation } | { OnUpdateSelectedCell: SelectedCellNavigation } | "OnWorldWillUpdate";

export type CameraNavigation = "Left" | "Right" | "Up" | "Down";

export type TextAlignment = "Center" | "Left" | "Right";

export interface UiViewState {
    selected_id: ViewComponentIds;
    selectable_ids: ViewComponentIds[];
    view_data: ViewDataTypes;
}

export type Ascii = "space" | "plus" | "tilde" | "doubleTilde";

export type UiSubscription = "UnknownUiSubscription";

export type UiEvents = { OnSelect: SelectionDirections } | { OnSelectById: ViewComponentIds } | { OnClick: ViewComponentIds } | { OnShortcut: string } | "OnCloseView";

export type EcsSubscriptionIds = "GeneralSubscription" | "ViewSubscription" | "PopupSubscription" | "GameMetaSubscription";

export type CellColors = { rgb: [number, number, number] };

export type AsciiIds = "sand" | "shallowWater" | "deepWater" | "unknownAsciiId";

export type WindowEvents = { Resize: [number, number] };

export type InputEvents = { Keyboard: KeyboardEvent } | { Mouse: MouseEvent };

export type KeyboardEvent = { OnPress: Keys };

export type MouseEvent = { OnPress: boolean } | { OnMove: [number, number] };

export type Keys = "UpArrow" | "DownArrow" | "LeftArrow" | "RightArrow" | "Enter" | "Esc" | "Backspace" | "Tab" | "BackTab" | { Char: string };

export type GameStatus = "GameDidNotStart" | "GameDidStart" | "GameDidPaused" | "GameWillEnded";

