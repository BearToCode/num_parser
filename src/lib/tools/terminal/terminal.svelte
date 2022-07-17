<script lang="ts">
	import { onMount } from 'svelte';
	import 'xterm/css/xterm.css';
	import * as xterm from 'xterm';
	import * as fit from 'xterm-addon-fit';
	import ResizeObserver from 'svelte-resize-observer';
	import termIO from './io';
	import '@styles/term.scss';
	import theme from '@utils/theme';
	import Sender from './sender.svelte';

	let terminalElement: HTMLElement;
	let terminalController: xterm.Terminal;
	let termFit: fit.FitAddon;
	let IO: termIO;

	let focusSender: () => void;

	export let hidden: boolean;

	function initializeXterm() {
		terminalController = new xterm.Terminal({
			fontFamily: 'JetBrainsMono',
			theme: {
				background: theme.colors.primary['700'],
			},
			convertEol: true,
		});
		termFit = new fit.FitAddon();
		terminalController.loadAddon(termFit);
		terminalController.open(terminalElement);
		termFit.fit();
		IO = new termIO(terminalController);
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
	class={`
		${hidden && 'absolute top-0 right-0 h-0 w-0 overflow-hidden'}
		${!hidden && 'absolute left-0 top-0 right-0 bottom-0'}
	`}
>
	<div
		id="terminal"
		bind:this={terminalElement}
		class={`absolute left-0 right-0 top-0 bottom-8`}
		on:click={focusSender}
	/>

	<div class={`absolute top-0 right-0 bottom-0 left-0`}>
		<ResizeObserver on:resize={handleTermResize} />
	</div>

	<Sender
		bind:focus={focusSender}
		on:command={(e) => {
			IO.execute(e.detail.command);
		}}
	/>
</div>

<svelte:window on:resize={handleTermResize} />
