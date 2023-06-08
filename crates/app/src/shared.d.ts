/* tslint:disable */
/* eslint-disable */
export interface Asset {
  id: AssetTypes;
  cells: Cell[];
  description: string | null;
}

export type AssetTypes = "wall" | "unknownAssetType";

export type WindowEvents = { Resize: [number, number] };

export type OptionMenuIds = "Title" | "OptionList" | "Back";

export type UiSubscription = "UnknownUiSubscription";

export type UiEvents =
  | { OnSelect: SelectionDirections }
  | { OnSelectById: ViewComponentIds }
  | { OnClick: ViewComponentIds };

export type TextAlignment = "Center" | "Left" | "Right";

export interface Cell {
  symbol: Ascii;
  background: CellColors;
  foreground: CellColors;
  isBold: boolean;
}

export interface Sprite {
  asset: Asset;
  frame: Rect;
}

export type SelectionDirections = "Next" | "Previous";

export type MainMenuIds = "Title" | "MenuList" | "NewGame" | "Options" | "Quit";

export interface UiViewState {
  selected_id: ViewComponentIds;
  selectable_ids: ViewComponentIds[];
}

export interface UiLayout {
  margin: number;
  alignment: LayoutAlignments;
  constraints: LayoutConstraints[];
}

export type RenderSubscription = { OnWorldDidUpdate: UiView | null };

export type RenderEvents = "OnWorldWillUpdate";

export type UiViewIds = "Main" | "Game" | "Options";

export interface UiList {
  id: ViewComponentIds;
  label: string;
  children: UiLabel[];
}

export interface Rect {
  x: number;
  y: number;
  width: number;
  height: number;
}

export type CellColors = "black" | "white";

export type ViewComponentIds =
  | { Main: MainMenuIds }
  | { Options: OptionMenuIds }
  | { Game: GameIds };

export type Ascii = "space" | "plus" | "minus" | "a";

export type LayoutConstraints =
  | { Percentage: number }
  | { Value: number }
  | { MinValue: number }
  | { MaxValue: number };

export type LayoutAlignments = "Horizontal" | "Vertical";

export interface Interaction {
  is_enabled: boolean;
  is_selected: boolean;
}

export type EcsEvents =
  | { Send: SendEvents }
  | { Subscriber: SubscriptionEvents };

export type SubscriptionEvents =
  | { General: GeneralSubscription }
  | { Ui: UiSubscription }
  | { Renderer: RenderSubscription };

export type SendEvents =
  | { General: GeneralEvents }
  | { Ui: UiEvents }
  | { Renderer: RenderEvents };

export type GameIds =
  | "Time"
  | "Turns"
  | "Canvas"
  | "Menu"
  | "Stones"
  | "Wood"
  | "Food"
  | "None";

export interface UiView {
  id: UiViewIds;
  layout: UiLayout;
  state: UiViewState;
  children: UiViewChild[];
}

export type UiViewChild =
  | { List: UiList }
  | { Label: UiLabel }
  | { Section: UiView }
  | "Placeholder"
  | { GameCanvas: Rect };

export interface UiLabel {
  id: ViewComponentIds;
  text: string;
  alignment: TextAlignment;
}

export type GeneralSubscription =
  | "OnApplicationDidStart"
  | "OnApplicationDidInitialise"
  | "OnApplicationDidClose";

export type GeneralEvents =
  | { OnApplicationResize: [number, number] }
  | { OnApplicationWillInitialise: [number, number] }
  | "OnApplicationWillClose";
