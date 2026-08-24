import Link from "next/link";
import { PageShell } from "@/components/PageShell";

export default function NotFound() {
  return (
    <PageShell
      current="/"
      hero={{
        human: {
          kicker: "Missing page",
          title: "This path does not exist.",
          body: "The page you requested is not part of the library site.",
        },
        machine: {
          kicker: "Next step",
          title: "Return to a known route.",
          body: "Use the navigation, or start from the home page.",
        },
      }}
    >
      <main className="paper" id="paper">
        <p className="page-lede">
          That URL is not in this static site. The pages below are the current
          public surface.
        </p>
        <p>
          <Link href="/">Back to home</Link>
        </p>
      </main>
    </PageShell>
  );
}
