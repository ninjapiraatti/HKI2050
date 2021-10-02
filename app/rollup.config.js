import path from 'path'
import { nodeResolve } from '@rollup/plugin-node-resolve'
import vue from 'rollup-plugin-vue'
import postcss from 'rollup-plugin-postcss'
import alias from '@rollup/plugin-alias'
import autoprefixer from 'autoprefixer'
import replace from '@rollup/plugin-replace'
import copy from 'rollup-plugin-copy-assets'

const production = !process.env.ROLLUP_WATCH

export default {
	input: 'src/main.js',
	output: {
		dir: '../public',
		format: 'iife',
		sourcemap: !production,
	},

	plugins: [
		alias({
			entries: [
				{ find: /^@root/, replacement: `${path.resolve(__dirname)}/src` },
				{ find: /^@(components|views|forms|store|transitions)/, replacement: `${path.resolve(__dirname)}/src/$1` },
				{ find: /^~/, replacement: `${path.resolve(__dirname)}/node_modules/` },
			],
		}),

		nodeResolve({ browser: true }),
		
		replace({
			preventAssignment: true,
			'process.env.NODE_ENV': JSON.stringify('development'),
			'__VUE_PROD_DEVTOOLS__': !production,
			'__VUE_OPTIONS_API__': true,
		}),
		
		vue(),

		postcss({
			extract: true,
			minimize: production,
			sourceMap: !production,
			plugins: [
				autoprefixer(),
			]
		}),

		copy({
			assets: [
				'assets',
				'index.html',
			],
		}),
	]
}
