export default {
  index: {
    type: "page",
    title: "Home",
    theme: {
      layout: "full",
      sidebar: false,
      toc: false,
      breadcrumb: false,
      pagination: false,
      timestamp: false,
    },
  },
  authoring: { type: "page", title: "Authoring" },
  schema: { type: "page", title: "Schema" },
  "for-llms": {
    type: "page",
    title: "For LLMs",
    theme: {
      sidebar: false,
      toc: false,
      breadcrumb: false,
      pagination: false,
      timestamp: false,
    },
  },
};
