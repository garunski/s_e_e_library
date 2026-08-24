import type { ReactNode } from "react";
import type { Metadata } from "next";
import { PageShell } from "@/components/PageShell";
import { authoringSection, schemaSection, type NavSection } from "@/lib/nav";

type InnerPageProps = {
  current: string;
  section: NavSection;
  human: {
    kicker: string;
    title: string;
    body: string;
  };
  machine: {
    kicker: string;
    title: string;
    body: string;
  };
  children: ReactNode;
};

export function innerMetadata(title: string, description: string): Metadata {
  return { title, description };
}

export function InnerPage({
  current,
  section,
  human,
  machine,
  children,
}: InnerPageProps) {
  return (
    <PageShell
      current={current}
      hero={{
        human: {
          kicker: human.kicker,
          title: human.title,
          body: human.body,
        },
        machine: {
          kicker: machine.kicker,
          title: machine.title,
          body: machine.body,
        },
      }}
      section={section}
    >
      <main className="paper" id="paper">
        {children}
      </main>
    </PageShell>
  );
}

export function AuthoringPage(props: Omit<InnerPageProps, "section">) {
  return <InnerPage {...props} section={authoringSection} />;
}

export function SchemaPage(props: Omit<InnerPageProps, "section">) {
  return <InnerPage {...props} section={schemaSection} />;
}
