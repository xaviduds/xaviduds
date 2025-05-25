export default {
  root: ".", // all files in root (index.html, assets/, style.css)
  build: {
    outDir: "dist",
    assetsDir: "assets",
    emptyOutDir: true,
    minify: "esbuild",
  },
};
