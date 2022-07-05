<script lang="ts">
	import { onMount } from 'svelte';
	import { FitAddon } from 'xterm-addon-fit';
	import { watchResize } from 'svelte-watch-resize';
	import resolveConfig from 'tailwindcss/resolveConfig';
	import tailwindConfig from 'tailwind.config.cjs';
	import _ from 'xterm/css/xterm.css';

	// tailwind
	const fullConfig = resolveConfig(tailwindConfig);

	let termElem: HTMLElement;
	let xterm: any;
	let fitAddon: any;

	onMount(async () => {
		xterm = await import('xterm');
		fitAddon = new FitAddon();
		let term = new xterm.Terminal({
			theme: {
				background: fullConfig.theme.colors.primary['700'],
			},
		});

		term.loadAddon(fitAddon);
		term.open(termElem);
		fitAddon.fit();
		term.write('Hello from \x1B[1;3;31mxterm.js\x1B[0m $ ');
	});

	function resizeTerminal() {
		if (fitAddon) {
			fitAddon.fit();
		}
	}
</script>

<div
	class="w-full overflow-y-scroll text-primary-200 flex-grow"
	id="terminal"
	bind:this={termElem}
	use:watchResize={resizeTerminal}
/>

<style lang="scss">
	#terminal,
	:global(.xterm .xterm-viewport) {
		/* width */
		&::-webkit-scrollbar {
			@apply w-3;
		}

		&:hover::-webkit-scrollbar-thumb {
			@apply bg-primary-600;
		}

		/* Handle on hover */
		&::-webkit-scrollbar-thumb:hover {
			@apply bg-primary-500;
		}
	}
</style>
