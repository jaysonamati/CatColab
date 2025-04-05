// vite.config.ts
import { nodeTypes } from "file:///home/amati/CodeProjects/Consulting/AISC/Autostructures/code/Live-catcolab/code/CatColab/packages/frontend/node_modules/@mdx-js/mdx/index.js";
import rehypeRaw from "file:///home/amati/CodeProjects/Consulting/AISC/Autostructures/code/Live-catcolab/code/CatColab/packages/frontend/node_modules/rehype-raw/index.js";
import { defineConfig } from "file:///home/amati/CodeProjects/Consulting/AISC/Autostructures/code/Live-catcolab/code/CatColab/packages/frontend/node_modules/vite/dist/node/index.js";
import solid from "file:///home/amati/CodeProjects/Consulting/AISC/Autostructures/code/Live-catcolab/code/CatColab/packages/frontend/node_modules/vite-plugin-solid/dist/esm/index.mjs";
import topLevelAwait from "file:///home/amati/CodeProjects/Consulting/AISC/Autostructures/code/Live-catcolab/code/CatColab/packages/frontend/node_modules/vite-plugin-top-level-await/exports/import.mjs";
import wasm from "file:///home/amati/CodeProjects/Consulting/AISC/Autostructures/code/Live-catcolab/code/CatColab/packages/frontend/node_modules/vite-plugin-wasm/exports/import.mjs";
import pkg from "file:///home/amati/CodeProjects/Consulting/AISC/Autostructures/code/Live-catcolab/code/CatColab/packages/frontend/node_modules/@vinxi/plugin-mdx/dist/index.cjs";
var { default: mdx } = pkg;
var vite_config_default = defineConfig({
  plugins: [
    wasm(),
    topLevelAwait(),
    mdx.withImports({})({
      jsx: true,
      jsxImportSource: "solid-js",
      providerImportSource: "solid-mdx",
      rehypePlugins: [[rehypeRaw, { passThrough: nodeTypes }]]
    }),
    solid({
      extensions: [".mdx", ".md"]
    })
  ],
  build: {
    chunkSizeWarningLimit: 2e3,
    sourcemap: false
  },
  server: {
    proxy: {
      "/api": {
        target: "http://localhost:8000",
        ws: true,
        changeOrigin: true,
        rewrite: (path) => path.replace(/^\/api/, "")
      }
    },
    watch: {
      usePolling: true
      // polling may be more reliable within the container
    }
  }
});
export {
  vite_config_default as default
};
//# sourceMappingURL=data:application/json;base64,ewogICJ2ZXJzaW9uIjogMywKICAic291cmNlcyI6IFsidml0ZS5jb25maWcudHMiXSwKICAic291cmNlc0NvbnRlbnQiOiBbImNvbnN0IF9fdml0ZV9pbmplY3RlZF9vcmlnaW5hbF9kaXJuYW1lID0gXCIvaG9tZS9hbWF0aS9Db2RlUHJvamVjdHMvQ29uc3VsdGluZy9BSVNDL0F1dG9zdHJ1Y3R1cmVzL2NvZGUvTGl2ZS1jYXRjb2xhYi9jb2RlL0NhdENvbGFiL3BhY2thZ2VzL2Zyb250ZW5kXCI7Y29uc3QgX192aXRlX2luamVjdGVkX29yaWdpbmFsX2ZpbGVuYW1lID0gXCIvaG9tZS9hbWF0aS9Db2RlUHJvamVjdHMvQ29uc3VsdGluZy9BSVNDL0F1dG9zdHJ1Y3R1cmVzL2NvZGUvTGl2ZS1jYXRjb2xhYi9jb2RlL0NhdENvbGFiL3BhY2thZ2VzL2Zyb250ZW5kL3ZpdGUuY29uZmlnLnRzXCI7Y29uc3QgX192aXRlX2luamVjdGVkX29yaWdpbmFsX2ltcG9ydF9tZXRhX3VybCA9IFwiZmlsZTovLy9ob21lL2FtYXRpL0NvZGVQcm9qZWN0cy9Db25zdWx0aW5nL0FJU0MvQXV0b3N0cnVjdHVyZXMvY29kZS9MaXZlLWNhdGNvbGFiL2NvZGUvQ2F0Q29sYWIvcGFja2FnZXMvZnJvbnRlbmQvdml0ZS5jb25maWcudHNcIjtpbXBvcnQgeyBub2RlVHlwZXMgfSBmcm9tIFwiQG1keC1qcy9tZHhcIjtcbmltcG9ydCByZWh5cGVSYXcgZnJvbSBcInJlaHlwZS1yYXdcIjtcbmltcG9ydCB7IGRlZmluZUNvbmZpZyB9IGZyb20gXCJ2aXRlXCI7XG5pbXBvcnQgc29saWQgZnJvbSBcInZpdGUtcGx1Z2luLXNvbGlkXCI7XG5pbXBvcnQgdG9wTGV2ZWxBd2FpdCBmcm9tIFwidml0ZS1wbHVnaW4tdG9wLWxldmVsLWF3YWl0XCI7XG5pbXBvcnQgd2FzbSBmcm9tIFwidml0ZS1wbHVnaW4td2FzbVwiO1xuXG4vLyBAdHMtZXhwZWN0LWVycm9yIFR5cGVzIGFyZSBtaXNzaW5nLlxuLy8gKkFsc28qLCB0aGlzIHBsdWdpbiBjYXVzZXMgVml0ZSA1IHRvIGNvbXBsYWluIGFib3V0IENKUy5cbi8vIGh0dHBzOi8vZ2l0aHViLmNvbS9ua3NhcmFmL3ZpbnhpL2lzc3Vlcy8yODlcbmltcG9ydCBwa2cgZnJvbSBcIkB2aW54aS9wbHVnaW4tbWR4XCI7XG5jb25zdCB7IGRlZmF1bHQ6IG1keCB9ID0gcGtnO1xuXG5leHBvcnQgZGVmYXVsdCBkZWZpbmVDb25maWcoe1xuICAgIHBsdWdpbnM6IFtcbiAgICAgICAgd2FzbSgpLFxuICAgICAgICB0b3BMZXZlbEF3YWl0KCksXG4gICAgICAgIG1keC53aXRoSW1wb3J0cyh7fSkoe1xuICAgICAgICAgICAganN4OiB0cnVlLFxuICAgICAgICAgICAganN4SW1wb3J0U291cmNlOiBcInNvbGlkLWpzXCIsXG4gICAgICAgICAgICBwcm92aWRlckltcG9ydFNvdXJjZTogXCJzb2xpZC1tZHhcIixcbiAgICAgICAgICAgIHJlaHlwZVBsdWdpbnM6IFtbcmVoeXBlUmF3LCB7IHBhc3NUaHJvdWdoOiBub2RlVHlwZXMgfV1dLFxuICAgICAgICB9KSxcbiAgICAgICAgc29saWQoe1xuICAgICAgICAgICAgZXh0ZW5zaW9uczogW1wiLm1keFwiLCBcIi5tZFwiXSxcbiAgICAgICAgfSksXG4gICAgXSxcbiAgICBidWlsZDoge1xuICAgICAgICBjaHVua1NpemVXYXJuaW5nTGltaXQ6IDIwMDAsXG4gICAgICAgIHNvdXJjZW1hcDogZmFsc2UsXG4gICAgfSxcbiAgICBzZXJ2ZXI6IHtcbiAgICAgICAgcHJveHk6IHtcbiAgICAgICAgICAgIFwiL2FwaVwiOiB7XG4gICAgICAgICAgICAgICAgdGFyZ2V0OiBcImh0dHA6Ly9sb2NhbGhvc3Q6ODAwMFwiLFxuICAgICAgICAgICAgICAgIHdzOiB0cnVlLFxuICAgICAgICAgICAgICAgIGNoYW5nZU9yaWdpbjogdHJ1ZSxcbiAgICAgICAgICAgICAgICByZXdyaXRlOiAocGF0aCkgPT4gcGF0aC5yZXBsYWNlKC9eXFwvYXBpLywgXCJcIiksXG4gICAgICAgICAgICB9LFxuICAgICAgICB9LFxuICAgICAgICB3YXRjaDoge1xuICAgICAgICAgICAgdXNlUG9sbGluZzogdHJ1ZSwgLy8gcG9sbGluZyBtYXkgYmUgbW9yZSByZWxpYWJsZSB3aXRoaW4gdGhlIGNvbnRhaW5lclxuICAgICAgICB9LFxuICAgIH0sXG59KTtcbiJdLAogICJtYXBwaW5ncyI6ICI7QUFBZ2YsU0FBUyxpQkFBaUI7QUFDMWdCLE9BQU8sZUFBZTtBQUN0QixTQUFTLG9CQUFvQjtBQUM3QixPQUFPLFdBQVc7QUFDbEIsT0FBTyxtQkFBbUI7QUFDMUIsT0FBTyxVQUFVO0FBS2pCLE9BQU8sU0FBUztBQUNoQixJQUFNLEVBQUUsU0FBUyxJQUFJLElBQUk7QUFFekIsSUFBTyxzQkFBUSxhQUFhO0FBQUEsRUFDeEIsU0FBUztBQUFBLElBQ0wsS0FBSztBQUFBLElBQ0wsY0FBYztBQUFBLElBQ2QsSUFBSSxZQUFZLENBQUMsQ0FBQyxFQUFFO0FBQUEsTUFDaEIsS0FBSztBQUFBLE1BQ0wsaUJBQWlCO0FBQUEsTUFDakIsc0JBQXNCO0FBQUEsTUFDdEIsZUFBZSxDQUFDLENBQUMsV0FBVyxFQUFFLGFBQWEsVUFBVSxDQUFDLENBQUM7QUFBQSxJQUMzRCxDQUFDO0FBQUEsSUFDRCxNQUFNO0FBQUEsTUFDRixZQUFZLENBQUMsUUFBUSxLQUFLO0FBQUEsSUFDOUIsQ0FBQztBQUFBLEVBQ0w7QUFBQSxFQUNBLE9BQU87QUFBQSxJQUNILHVCQUF1QjtBQUFBLElBQ3ZCLFdBQVc7QUFBQSxFQUNmO0FBQUEsRUFDQSxRQUFRO0FBQUEsSUFDSixPQUFPO0FBQUEsTUFDSCxRQUFRO0FBQUEsUUFDSixRQUFRO0FBQUEsUUFDUixJQUFJO0FBQUEsUUFDSixjQUFjO0FBQUEsUUFDZCxTQUFTLENBQUMsU0FBUyxLQUFLLFFBQVEsVUFBVSxFQUFFO0FBQUEsTUFDaEQ7QUFBQSxJQUNKO0FBQUEsSUFDQSxPQUFPO0FBQUEsTUFDSCxZQUFZO0FBQUE7QUFBQSxJQUNoQjtBQUFBLEVBQ0o7QUFDSixDQUFDOyIsCiAgIm5hbWVzIjogW10KfQo=
