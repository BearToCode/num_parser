<script lang="ts">
	import Icon from 'svelte-icons-pack';
	import BiTerminal from 'svelte-icons-pack/bi/BiTerminal';

	import Chrome from './nav/chrome.svelte';
	import Navbar from './nav/navbar.svelte';
	import Toolbar from './nav/toolbar.svelte';
	import MainWindow from './main-window.svelte';

	let windows: { name: string; icon: any }[] = [{ name: 'Terminal', icon: BiTerminal }];

	let toggleToolsWindow: () => void;
</script>

<div
	class="inline-grid w-screen h-screen overflow-hidden
  grid-cols-[min-content_1fr] grid-rows-[min-content_1fr]"
>
	<Chrome />

	<Navbar>
		<slot name="links" />
	</Navbar>

	<MainWindow bind:toggleToolsWindow>
		<slot name="routes" />
	</MainWindow>

	<Toolbar>
		{#each windows as window}
			<button class="flex items-center" on:click={toggleToolsWindow}>
				<div class="text-lg mr-1">
					<Icon src={window.icon} color="currentcolor" />
				</div>
				{window.name}
			</button>
		{/each}
	</Toolbar>
</div>
