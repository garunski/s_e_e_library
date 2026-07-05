import nextra from "nextra";

const basePath = process.env.BASE_PATH ?? "/s_e_e_library";

const withNextra = nextra({
  defaultShowCopyCode: true,
});

export default withNextra({
  output: "export",
  basePath,
  assetPrefix: basePath,
  images: { unoptimized: true },
  env: { NEXT_PUBLIC_BASE_PATH: basePath },
});
