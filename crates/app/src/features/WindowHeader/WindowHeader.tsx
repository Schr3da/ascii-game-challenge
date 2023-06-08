import { Platforms } from "../../services";
import { MacosHeader } from "./Macos";
import { WindowsHeader } from "./Windows";
import { WindowHeaderProps } from "./WindowHeader.types";

export const WindowHeader = ({ className, platform }: WindowHeaderProps) => {
  switch (platform) {
    case Platforms.Web:
    case Platforms.Android:
    case Platforms.iOS:
      return null;
    case Platforms.Macos:
      return <MacosHeader className={className} />;
    default:
      return <WindowsHeader className={className} />;
  }
};
