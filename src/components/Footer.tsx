import Link from "next/link";
import { CurrentYear } from "@/components/CurrentYear";
import { assetPath, brandLockup, footerLinks, siteName } from "@/lib/nav";

export function Footer() {
  return (
    <footer className="site-footer">
      <div className="footer-shell">
        <Link className="footer-mark" href="/" aria-label={`${siteName} home`}>
          <img src={assetPath("/logo.svg")} width={44} height={44} alt="" />
          <span className="brand-lockup">
            <strong>{brandLockup.primary}</strong>
            <small>{brandLockup.secondary}</small>
          </span>
        </Link>
        <p>The package catalog S.E.E. installs from.</p>
        <nav className="footer-links" aria-label="S.E.E. links">
          {footerLinks.map((link) => (
            <a key={link.href} href={link.href} rel="noreferrer">
              {link.label} <span aria-hidden="true">↗</span>
            </a>
          ))}
        </nav>
      </div>
      <div className="footer-bottom">
        <p>The catalog and its authoring contract.</p>
        <p className="footer-legal">
          <span aria-hidden="true">©</span> <CurrentYear />{" "}
          <a href="https://garunski.com" rel="noreferrer">
            Garunski
          </a>
        </p>
        <a href="#top">
          Back to top <span aria-hidden="true">↑</span>
        </a>
      </div>
    </footer>
  );
}
