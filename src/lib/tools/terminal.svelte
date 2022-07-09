<script lang="ts">
	import { onMount } from 'svelte';
	import 'xterm/css/xterm.css';
	import * as xterm from 'xterm';
	import * as fit from 'xterm-addon-fit';
	import ResizeObserver from 'svelte-resize-observer';

	let terminalElement: HTMLElement;
	let terminalController: xterm.Terminal;
	let termFit: fit.FitAddon;

	function initializeXterm() {
		terminalController = new xterm.Terminal();
		termFit = new fit.FitAddon();
		terminalController.loadAddon(termFit);
		terminalController.open(terminalElement);
		termFit.fit();
		terminalController.write(
			'\x1b[32mWelcome to Warp code!Welcome to Warp code!Welcome to Warp code!Welcome to Warp code!Welcome to Warp code!Welcome to Warp code!Welcome to Warp code!Welcome to Warp code!Welcome to Warp code!Welcome to Warp code!Welcome to Warp code!Welcome to Warp code!Welcome to Warp code!\x1b[m\n'
		);
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

<div id="terminal" bind:this={terminalElement} />

<div class="absolute top-0 left-0 right-0 bottom-0">
	<ResizeObserver on:resize={handleTermResize} />
</div>

<svelte:window on:resize={handleTermResize} />
