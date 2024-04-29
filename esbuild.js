import esbuild from 'esbuild';

esbuild.build({
  entryPoints: ['package.js'],
  bundle: true,
  outfile: 'src/runtime.js',
  format: 'esm',
  minify: true,
}).catch(() => process.exit(1));
