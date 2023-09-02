import { ComposerProps } from "./Composer.types";

export const Composer = ({ children, components }: ComposerProps) =>
  components.reduceRight((result, next) => {
    const [Component, props] = Array.isArray(next)
      ? [next[0], next[1]]
      : [next, {}];

    return <Component {...(props as object)}>{result}</Component>;
  }, children);
