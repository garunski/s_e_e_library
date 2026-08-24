import Link from "next/link";
import {
  assetPath,
  brandLockup,
  headerCta,
  isPrimaryCurrent,
  primaryNav,
  siteName,
  type NavLink,
} from "@/lib/nav";

type HeaderProps = {
  current: string;
};

function NavItem({ link, current }: { link: NavLink; current: string }) {
  const currentAttr = isPrimaryCurrent(link.href, current) ? "page" : undefined;
  if (link.file || link.external) {
    return (
      <a
        href={link.file ? assetPath(link.href) : link.href}
        aria-current={currentAttr}
        rel={link.external ? "noreferrer" : undefined}
      >
        {link.label}
      </a>
    );
  }
  return (
    <Link href={link.href} aria-current={currentAttr}>
      {link.label}
    </Link>
  );
}

export function Header({ current }: HeaderProps) {
  return (
    <header className="site-header">
      <Link className="site-brand" href="/" aria-label={`${siteName} home`}>
        <img src={assetPath("/logo.svg")} width={44} height={44} alt="" />
        <span className="brand-lockup">
          <strong>{brandLockup.primary}</strong>
          <small>{brandLockup.secondary}</small>
        </span>
      </Link>
      <input
        className="nav-toggle-input"
        id="site-nav-toggle"
        type="checkbox"
      />
      <label className="nav-toggle" htmlFor="site-nav-toggle">
        Menu
      </label>
      <nav className="primary-nav" aria-label="Primary navigation">
        {primaryNav.map((link) => (
          <NavItem key={link.href} link={link} current={current} />
        ))}
      </nav>
      <a className="header-cta" href={assetPath(headerCta.href)}>
        {headerCta.label}
      </a>
    </header>
  );
}
