import Link from "next/link";
import { PageShell } from "@/components/PageShell";
import { assetPath } from "@/lib/nav";

export const metadata = {
  title: {
    absolute: "S.E.E. Official Library",
  },
  description:
    "Versioned workflows, prompts, skills, commands, and bundles for hub and spoke projects.",
};

export default function HomePage() {
  return (
    <PageShell
      variant="home"
      current="/"
      skipHref="#main"
      hero={{
        variant: "home",
        human: {
          kicker: "Reusable machinery",
          title: "A package is how reuse ships.",
          titleId: "human-heading",
          body: (
            <p className="hero-lede">
              Workflows, prompts, skills, and commands only travel between
              projects once they are named, versioned, and installable.
            </p>
          ),
        },
        machine: {
          kicker: "Official Library",
          title: "S.E.E. installs from this catalog.",
          titleId: "system-heading",
          body: (
            <p className="hero-lede">
              The in-app Library reads catalog.json as the install map. Payload
              files live under packages, and the schemas decide what is valid.
            </p>
          ),
          extra: (
            <div className="hero-actions">
              <a className="button button-primary" href={assetPath("/catalog.json")}>
                Open catalog.json
              </a>
              <Link className="button button-secondary" href="/authoring/">
                Author a package
              </Link>
            </div>
          ),
        },
        scrollCue: { href: "#main", label: "Package types" },
      }}
    >
      <main className="home-main" id="main">
        <section className="machinery-band" aria-labelledby="pieces-heading">
          <div className="home-section">
            <div className="section-intro section-intro-wide">
              <p className="section-kicker">Package types</p>
              <h2 id="pieces-heading">Five package types carry the machinery.</h2>
              <p className="section-lede">
                Each type has a payload shape, an install destination, and an
                authoring guide. Bundles install several of them together.
              </p>
            </div>

            <ol className="product-grid">
              <li>
                <Link href="/authoring/workflows/">
                  <span className="piece-number">01</span>
                  <h3>Workflows</h3>
                  <p>Multi-step task definitions the engine runs against a project.</p>
                </Link>
              </li>
              <li>
                <Link href="/authoring/prompts/">
                  <span className="piece-number">02</span>
                  <h3>Prompts</h3>
                  <p>Named system instruction referenced by workflows and commands.</p>
                </Link>
              </li>
              <li>
                <Link href="/authoring/skills/">
                  <span className="piece-number">03</span>
                  <h3>Skills</h3>
                  <p>Activatable guides that teach a worker a repeatable capability.</p>
                </Link>
              </li>
              <li>
                <Link href="/authoring/commands/">
                  <span className="piece-number">04</span>
                  <h3>Commands</h3>
                  <p>Worker definitions that map a task to an external agent CLI.</p>
                </Link>
              </li>
              <li>
                <Link href="/authoring/bundles/">
                  <span className="piece-number">05</span>
                  <h3>Bundles</h3>
                  <p>Curated install maps that deliver several linked packages together.</p>
                </Link>
              </li>
            </ol>

            <div className="library-note">
              <p>
                After the payload exists, Publish is the catalog entry, the
                install map, and the validation step.
              </p>
              <Link href="/authoring/publish/">
                Publish a package
              </Link>
            </div>
          </div>
        </section>

        <section className="home-section" aria-labelledby="contract-heading">
          <div className="section-intro">
            <p className="section-kicker">The contract</p>
            <h2 id="contract-heading">Schema pages own the payload shape.</h2>
            <p className="section-lede">
              Every schema page carries its JSON Schema file, downloadable under
              /schema/. Authoring guides explain how to write a package that
              satisfies one.
            </p>
          </div>
          <div className="external-actions">
            <Link className="button button-dark" href="/schema/">
              Catalog schema
            </Link>
            <a href={assetPath("/llms.txt")}>
              Everything as one text file
            </a>
          </div>
        </section>
      </main>
    </PageShell>
  );
}
