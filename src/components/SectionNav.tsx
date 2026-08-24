import Link from "next/link";
import { isCurrentHref, type NavSection } from "@/lib/nav";

type SectionNavProps = {
  section: NavSection;
  current: string;
};

export function SectionNav({ section, current }: SectionNavProps) {
  return (
    <nav className="section-nav" aria-label={section.ariaLabel}>
      <div className="section-nav-inner">
        <span className="section-label">{section.label}</span>
        {section.links.map((link) => (
          <Link
            key={link.href}
            href={link.href}
            aria-current={isCurrentHref(link.href, current) ? "page" : undefined}
          >
            {link.label}
          </Link>
        ))}
      </div>
    </nav>
  );
}
