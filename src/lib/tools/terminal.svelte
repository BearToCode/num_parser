<script lang="ts">
	import { onMount } from 'svelte';
	import 'xterm/css/xterm.css';
	import * as xterm from 'xterm';
	import * as fit from 'xterm-addon-fit';
	import { watchResize } from 'svelte-watch-resize';
	// Tailwind
	import resolveConfig from 'tailwindcss/resolveConfig';
	import tailwindConfig from 'tailwind.config.cjs';
	const fullConfig = resolveConfig(tailwindConfig);

	let terminalElement: HTMLElement;
	let terminalController: xterm.Terminal;
	let termFit: fit.FitAddon;

	function initializeXterm() {
		terminalController = new xterm.Terminal({
			theme: {
				background: fullConfig.theme.colors.primary['700'],
			},
		});
		termFit = new fit.FitAddon();
		terminalController.loadAddon(termFit);
		terminalController.open(terminalElement);
		for (let i = 0; i < 20; i++) {
			terminalController.write('\x1b[32mThis is a working terminal!\x1b[m\n');
			termFit.fit();
		}
		terminalController.onData((e) => {
			terminalController.write(e);
		});
	}
	onMount(async () => {
		initializeXterm();
	});
	function handleTermResize() {
		if (termFit) {
			termFit.fit();
		}
	}
</script>

<div
	id="terminal"
	bind:this={terminalElement}
	class="absolute top-0 bottom-0 right-0 left-0"
	use:watchResize={handleTermResize}
/>

<svelte:window on:resize={handleTermResize} />

<style lang="scss">
	:global(.xterm-viewport) {
		/* width */
		&::-webkit-scrollbar {
			@apply w-3;
		}
		/* FIXME: */
		/* Handle on hover */
		&::-webkit-scrollbar-thumb:hover {
			@apply bg-primary-400;
		}
	}
	:global(#terminal:hover .xterm-viewport::-webkit-scrollbar-thumb) {
		@apply bg-primary-600;
	}
</style>
