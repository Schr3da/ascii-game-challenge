import { ComponentType, JSXElementConstructor, PropsWithChildren } from "react"

type Component =
  | ComponentType
  | [JSXElementConstructor<PropsWithChildren<unknown>>, unknown];

export type ComposerProps = PropsWithChildren & {
  components: Component[];
}