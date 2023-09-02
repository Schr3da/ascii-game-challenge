import { Platforms } from "../../services";

export type ApplicationContextValue = {
  platform: Platforms;
  windowWidth: number;
  windowHeight: number;
};
