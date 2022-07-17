<script lang="ts">
	import Icon from 'svelte-icons-pack';
	import BiTerminal from 'svelte-icons-pack/bi/BiTerminal';

	import Chrome from './nav/chrome.svelte';
	import Navbar from './nav/navbar.svelte';
	import Toolbar from './nav/toolbar.svelte';
	import MainWindow from './main-window.svelte';
	import Rbar from './nav/rbar.svelte';

	import Terminal from './tools/terminal/terms-manager.svelte';

	type window = {
		name: string;
		icon: any;
		component: any;
	};

	let windows: window[] = [{ name: 'Terminal', icon: BiTerminal, component: Terminal }];

	let toggleToolsWindow: () => void;
	let setToolsWindow: (w: window) => void;
</script>

<div
	class="inline-grid w-screen h-screen overflow-hidden
  grid-cols-[min-content_1fr_min-content] grid-rows-[min-content_1fr_min-content] bg-primary-600"
>
	<Chrome />

	<Navbar>
		<slot name="links" />
	</Navbar>

	<MainWindow bind:toggleToolsWindow bind:setToolsWindow>
		<slot name="routes" />
	</MainWindow>

	<Rbar />

	<Toolbar>
		{#each windows as window}
			<button
				class="flex items-center"
				on:click={() => {
					setToolsWindow(window);
					toggleToolsWindow();
				}}
			>
				<div class="text-lg mr-1">
					<Icon src={window.icon} color="currentcolor" />
				</div>
				{window.name}
			</button>
		{/each}
	</Toolbar>

	<div class="bg-primary-700 border-t-2 border-l-2 border-primary-800" />
</div>
