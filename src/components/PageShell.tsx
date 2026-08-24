import type { ReactNode } from "react";
import { Footer } from "@/components/Footer";
import { Header } from "@/components/Header";
import { Hero, type HeroProps } from "@/components/Hero";
import { SectionNav } from "@/components/SectionNav";
import type { NavSection } from "@/lib/nav";

type PageShellProps = {
  variant?: "home" | "page";
  current: string;
  skipHref?: string;
  hero: HeroProps;
  section?: NavSection;
  children: ReactNode;
};

export function PageShell({
  variant = "page",
  current,
  skipHref = "#paper",
  hero,
  section,
  children,
}: PageShellProps) {
  return (
    <div className={variant}>
      <a className="skip" href={skipHref}>
        Skip to content
      </a>
      <Header current={current} />
      {section ? <SectionNav section={section} current={current} /> : null}
      <Hero {...hero} />
      {children}
      <Footer />
    </div>
  );
}
