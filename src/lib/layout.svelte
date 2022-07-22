<script lang="ts">
	// Icons
	import Icon from 'svelte-icons-pack';

	// Layout components
	import Chrome from './nav/chrome.svelte';
	import Navbar from './nav/navbar.svelte';
	import Toolbar from './nav/toolbar.svelte';
	import MainWindow from './main-window.svelte';
	import Rbar from './nav/rbar.svelte';

	// Windows
	import { toolsWindows, toggleToolsWindow, currentToolsWindow } from './windows-manager';
</script>

<div
	class="inline-grid w-screen h-screen overflow-hidden
  grid-cols-[min-content_1fr_min-content] grid-rows-[min-content_1fr_min-content] bg-primary-600"
>
	<Chrome />

	<Navbar>
		<slot name="links" />
	</Navbar>

	<MainWindow>
		<slot name="routes" />
	</MainWindow>

	<Rbar />

	<Toolbar>
		{#each toolsWindows as window}
			<button
				class="flex items-center"
				on:click={() => {
					currentToolsWindow.set(window);
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
