import type { Metadata } from "next";
import { Autour_One, Commissioner } from "next/font/google";
import "@/styles/site.css";

const autourOne = Autour_One({
  weight: "400",
  subsets: ["latin"],
  variable: "--font-autour-one",
  display: "swap",
});

const commissioner = Commissioner({
  weight: ["400", "600"],
  subsets: ["latin"],
  variable: "--font-commissioner",
  display: "swap",
});

export const metadata: Metadata = {
  title: {
    default: "S.E.E. Official Library",
    template: "%s - S.E.E. Official Library",
  },
  description:
    "Versioned workflows, prompts, skills, commands, and bundles for hub and spoke projects.",
  icons: {
    icon: [
      { url: "/logo-32.png", sizes: "32x32", type: "image/png" },
      { url: "/logo.svg", type: "image/svg+xml" },
    ],
    apple: [{ url: "/logo-128.png", sizes: "128x128", type: "image/png" }],
  },
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en" className={`${autourOne.variable} ${commissioner.variable}`}>
      <body>{children}</body>
    </html>
  );
}
