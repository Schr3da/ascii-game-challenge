import { Platforms } from "../../services";
import { WindowHeaderProps } from "./WindowHeader.types";
import { CommonHeader } from "./CommonHeader";

export const WindowHeader = ({ className, platform }: WindowHeaderProps) => {
  switch (platform) {
    case Platforms.Web:
    case Platforms.Android:
    case Platforms.iOS:
      return null;
    case Platforms.Linux:
    case Platforms.Windows:
    case Platforms.Macos:
      return <CommonHeader className={className} />;
  }
};
