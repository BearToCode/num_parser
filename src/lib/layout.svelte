<script lang="ts">
	const Icon = require('svelte-icons-pack');
	const BiTerminal = require('svelte-icons-pack/bi/BiTerminal');

	import Chrome from './navigation/chrome.svelte';
	import Navbar from './navigation/navbar.svelte';
	import Toolbar from './navigation/toolbar.svelte';
	import MainWindow from './main-window.svelte';

	let windows: { name: string; icon: any; position: windowPosition }[] = [
		{ name: 'Terminal', icon: BiTerminal, position: 'bottom' },
	];

	// let selectedWindow: string = '';

	type windowPosition = 'bottom' | 'left' | 'right';
</script>

<div
	class="inline-grid w-screen h-screen overflow-hidden
  grid-cols-[min-content_1fr] grid-rows-[min-content_1fr]"
>
	<Chrome />

	<Navbar>
		<slot name="links" />
	</Navbar>

	<MainWindow>
		<slot name="routes" />
	</MainWindow>

	<Toolbar>
		{#each windows as window}
			{#if window.position == 'bottom'}
				<button class="flex items-center">
					<div class="text-lg mr-1">
						<Icon src={window.icon} color="currentcolor" />
					</div>
					{window.name}
				</button>
			{/if}
		{/each}
	</Toolbar>
</div>
