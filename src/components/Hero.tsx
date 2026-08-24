import type { ReactNode } from "react";

export type HeroSide = {
  kicker: string;
  title: string;
  titleId?: string;
  body: ReactNode;
  extra?: ReactNode;
};

export type HeroProps = {
  variant?: "page" | "home";
  human: HeroSide;
  machine: HeroSide;
  scrollCue?: {
    href: string;
    label: string;
  };
};

function HeroCopy({
  side,
  className,
}: {
  side: HeroSide;
  className: string;
}) {
  return (
    <div className={className}>
      <p className="kicker">{side.kicker}</p>
      <h1 id={side.titleId}>{side.title}</h1>
      {typeof side.body === "string" ? <p>{side.body}</p> : side.body}
      {side.extra}
    </div>
  );
}

export function Hero({ variant = "page", human, machine, scrollCue }: HeroProps) {
  const copyClass = variant === "home" ? "hero-copy" : undefined;
  return (
    <div className={variant === "home" ? "rift home-hero" : "rift"} id="top">
      <HeroCopy
        className={copyClass ? `human ${copyClass}` : "human"}
        side={human}
      />
      <HeroCopy
        className={copyClass ? `machine ${copyClass}` : "machine"}
        side={machine}
      />
      {scrollCue ? (
        <a className="scroll-cue" href={scrollCue.href}>
          <span>{scrollCue.label}</span>
          <span aria-hidden="true">↓</span>
        </a>
      ) : null}
    </div>
  );
}
