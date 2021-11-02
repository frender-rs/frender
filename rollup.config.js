import { rollupOptions } from "@tlib/scripts/rollup";

export default rollupOptions({
  bundle: true,
  node: true,
  dts: true,
  copyMeta: true,
});
