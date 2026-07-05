import { Footer, Layout, Navbar } from "nextra-theme-docs";
import { Head } from "nextra/components";
import { getPageMap } from "nextra/page-map";
import { Year } from "../components/year";
import { ThemeSwitcher } from "../components/theme-switcher";
import "nextra-theme-docs/style.css";
import "../styles/globals.css";

const base = process.env.NEXT_PUBLIC_BASE_PATH ?? "";

export const metadata = {
  title: {
    default: "S.E.E. Official Library",
    template: "%s - S.E.E. Official Library",
  },
  description:
    "Versioned packages of workflows, prompts, skills, commands, and bundles for S.E.E. hub and spoke projects.",
};

const navbar = (
  <Navbar
    logo={
      <span className="see-logo">
        <img src={`${base}/logo.svg`} alt="S.E.E." width={24} height={24} />
        <b>S.E.E. Official Library</b>
      </span>
    }
    projectLink="https://github.com/garunski/s_e_e_library"
  />
);

export default async function RootLayout({ children }) {
  const pageMap = await getPageMap();
  return (
    <html lang="en" dir="ltr" suppressHydrationWarning>
      <Head />
      <body>
        <Layout
          navbar={navbar}
          footer={
            <Footer>
              <div className="see-footer">
                <p className="see-footer__copy">
                  &copy; <Year />{" "}
                  Garunski &middot; AGPL-3.0-only
                </p>
                <ThemeSwitcher />
              </div>
            </Footer>
          }
          pageMap={pageMap}
          docsRepositoryBase="https://github.com/garunski/s_e_e_library/tree/main"
          sidebar={{ defaultMenuCollapseLevel: 2 }}
          darkMode={false}
        >
          {children}
        </Layout>
      </body>
    </html>
  );
}
