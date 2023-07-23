/* tslint:disable */
/* eslint-disable */
export interface SelectedCell {
    top: number;
    bottom: number;
    frame: Rect;
    cell: Cell;
}

export interface Position {
    x: number;
    y: number;
}

export type UiSubscription = "UnknownUiSubscription";

export type UiEvents = { OnSelect: SelectionDirections } | { OnSelectById: ViewComponentIds } | { OnClick: ViewComponentIds } | "OnCloseView";

export type EcsSubscriptionIds = "GeneralSubscription" | "ViewSubscription" | "PopupSubscription" | "GameStatusSubscription";

export type MainMenuIds = "Title" | "MenuList" | "NewGame" | "Options" | "Quit";

export type CommandSubscription = { OnCommandDidUpdate: string[] };

export type CommandInputEvents = "New" | "Pop" | { Push: string } | { Execute: string[] } | "Cancel";

export type TextAlignment = "Center" | "Left" | "Right";

export type CellColors = { rgb: [number, number, number] };

export type RenderSubscription = { OnWorldDidUpdate: [UiView | null, UiView | null, GameStatus] };

export type RenderEvents = { OnUpdateCamera: CameraNavigation } | { OnUpdateSelectedCell: SelectedCellNavigation } | "OnWorldWillUpdate";

export type UiPopupViewIds = "Command" | "QuickAction";

export type UiViewIds = "Main" | "Game" | "Options" | { Popup: UiPopupViewIds };

export type ViewComponentIds = { Main: MainMenuIds } | { Options: OptionMenuIds } | { Game: GameIds } | { CommandPopup: CommandIds } | "QuickAction";

export type SelectionDirections = "Next" | "Previous";

export interface UiLayout {
    margin: number;
    alignment: LayoutAlignments;
    constraints: LayoutConstraints[];
}

export interface Sprite {
    asset: Asset;
    frame: Rect;
}

export interface Rect {
    x: number;
    y: number;
    width: number;
    height: number;
}

export type WindowEvents = { Resize: [number, number] };

export interface UiList {
    id: ViewComponentIds;
    label: string;
    children: UiLabel[];
}

export interface Cell {
    id: AsciiIds;
    symbol: Ascii;
    background: CellColors;
    foreground: CellColors;
    isBold: boolean;
}

export interface Asset {
    id: AssetTypes;
    cells: Cell[];
    description: string | null;
}

export type AssetTypes = "wall" | "unknownAssetType";

export type AsciiIds = "sand" | "shallowWater" | "deepWater" | "unknownAsciiId";

export type GameStatus = "GameDidNotStart" | "GameDidStart" | "GameDidPaused" | "GameWillEnded";

export type CameraNavigation = "Left" | "Right" | "Up" | "Down";

export interface GameViewFooterState {}

export interface GameViewHeaderState {
    currentDays: string;
    currentHours: string;
    currentMinutes: string;
    tickCount: string;
}

export type Ascii = "space" | "plus" | "tilde" | "doubleTilde";

export type GameIds = "Day" | "Time" | "Turns" | "Canvas" | "Menu" | "Stones" | "Wood" | "Food" | "None";

export interface Interaction {
    is_enabled: boolean;
    is_selected: boolean;
}

export type LayoutConstraints = { Percentage: number } | { Value: number } | { MinValue: number } | { MaxValue: number };

export type LayoutAlignments = "Horizontal" | "Vertical";

export type EcsEvents = { Send: SendEvents } | { Subscriber: SubscriptionEvents };

export type SubscriptionEvents = { General: GeneralSubscription } | { Ui: UiSubscription } | { Renderer: RenderSubscription } | { Command: CommandSubscription };

export type SendEvents = { General: GeneralEvents } | { Ui: UiEvents } | { Commands: CommandInputEvents } | { QuickAction: QuickActionEvents } | { Renderer: RenderEvents } | "Tick";

export type CommandIds = "Move" | "Build" | "Inspect" | "UnknownCommandId";

export type ViewDataTypes = "NoViewData" | "QuickActionData" | { GameHeader: GameViewHeaderState } | { CommandPopup: CommandPopupState };

export interface UiView {
    id: UiViewIds;
    layout: UiLayout;
    state: UiViewState;
    children: UiViewChild[];
}

export type UiViewChild = { List: UiList } | { Label: UiLabel } | { Section: UiView } | "Placeholder" | { GameCanvas: [[Cell, Position][], SelectedCell | null] };

export type QuickNavigationSubscription = { OnQuickActionDidUpdate: string };

export type QuickActionEvents = "New" | "Execute" | "Cancel";

export interface UiViewState {
    selected_id: ViewComponentIds;
    selectable_ids: ViewComponentIds[];
    view_data: ViewDataTypes;
}

export type SelectedCellNavigation = "Up" | "Down" | "Left" | "Right" | { Custom: [number, number] };

export type GeneralSubscription = "OnApplicationDidStart" | { OnApplicationDidLoadAssets: Record<AsciiIds, Cell> } | "OnApplicationDidInitialise" | "OnApplicationDidClose";

export type GeneralEvents = { OnApplicationResize: [number, number] } | { OnApplicationWillInitialise: [number, number] } | "OnApplicationWillClose";

export type OptionMenuIds = "Title" | "OptionList" | "LevelOfDifficulty" | "Sound" | "Back";

export interface UiLabel {
    id: ViewComponentIds;
    text: string;
    alignment: TextAlignment;
}

export interface CommandPopupState {
    currentSelectedGameTile: SelectedCell;
}

