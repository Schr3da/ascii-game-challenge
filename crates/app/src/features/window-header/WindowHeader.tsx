import { Platforms } from "../../services";
import { MacosHeader } from "./macos";
import { WindowsHeader } from "./windows";
import { WindowHeaderProps } from "./WindowHeader.types";

export const WindowHeader = ({ platform }: WindowHeaderProps) => {
  switch (platform) {
    case Platforms.Web:
    case Platforms.Android:
    case Platforms.iOS:
      return null;
    case Platforms.Macos:
      return <MacosHeader />;
    default:
      return <WindowsHeader />;
  }
};
