<script lang="ts">
	import { onMount } from 'svelte';
	import 'xterm/css/xterm.css';
	import * as xterm from 'xterm';
	import * as fit from 'xterm-addon-fit';
	import ResizeObserver from 'svelte-resize-observer';
	import { invoke } from '@tauri-apps/api/tauri';
	import termIO from './io';

	// Tailwind
	import resolveConfig from 'tailwindcss/resolveConfig';
	import tailwindConfig from 'tailwind.config.cjs';
	const fullConfig = resolveConfig(tailwindConfig);

	let terminalElement: HTMLElement;
	let terminalController: xterm.Terminal;
	let termFit: fit.FitAddon;
	let io: termIO;

	function initializeXterm() {
		terminalController = new xterm.Terminal({
			fontFamily: 'JetBrainsMono',
			theme: {
				background: fullConfig.theme.colors.primary['700'],
			},
			convertEol: true,
		});
		termFit = new fit.FitAddon();
		terminalController.loadAddon(termFit);
		terminalController.open(terminalElement);
		termFit.fit();
		io = new termIO(terminalController);
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

<div id="terminal" bind:this={terminalElement} class="absolute top-0 bottom-0 right-0 left-0" />

<div class="absolute top-0 right-0 bottom-0 left-0">
	<ResizeObserver on:resize={handleTermResize} />
</div>

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
