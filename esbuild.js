import esbuild from 'esbuild';

esbuild.build({
  entryPoints: ['package.tsx'],
  bundle: true,
  outfile: 'src/runtime.js',
  format: 'esm',
  minify: true,
}).catch(() => process.exit(1));
