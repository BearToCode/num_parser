<script lang="ts">
	// import Icon from 'svelte-icons-pack/Icon.svelte';
	import Icon from 'svelte-icons-pack/Icon.svelte';
	import VscChromeClose from 'svelte-icons-pack/vsc/VscChromeClose';
	import VscChromeRestore from 'svelte-icons-pack/vsc/VscChromeRestore';
	import VscChromeMinimize from 'svelte-icons-pack/vsc/VscChromeMinimize';
	import VscChromeMaximize from 'svelte-icons-pack/vsc/VscChromeMaximize';
	import { appWindow } from '@tauri-apps/api/window';
	import { useLocation } from 'svelte-navigator';

	const currentLocation = useLocation();

	async function windowIsMaximized() {
		return await appWindow.isMaximized();
	}

	// Get initial value
	let isMaximizedPromise = windowIsMaximized();

	// Update on window resize
	window.addEventListener('resize', () => {
		isMaximizedPromise = windowIsMaximized();
	});
</script>

<div
	data-tauri-drag-region
	class="h-8 bg-primary-200 toolbar text-neutral-400 flex flex-row col-span-2
  items-center justify-between text-sm border-b border-neutral-600 z-10"
>
	<div class="flex">
		<ul class="space-x-4 flex flex-row select-none w-min ml-4">
			<li>File</li>
			<li>Edit</li>
			<li>View</li>
			<li>Help</li>
		</ul>

		<div class="bg-primary-300 rounded-lg px-3 text-md text-gray-500 ml-8">
			{$currentLocation.pathname}
		</div>
	</div>

	<div class="text-base flex h-full">
		<button class="h-full hover:bg-neutral-700 hover:text-white px-3" on:click={() => appWindow.minimize()}>
			<Icon src={VscChromeMinimize} />
		</button>

		{#await isMaximizedPromise}
			<button class="h-full hover:bg-neutral-700 hover:text-white px-3" on:click={() => appWindow.maximize()}>
				<Icon src={VscChromeMaximize} />
			</button>
		{:then response}
			{#if response}
				<button class="h-full hover:bg-neutral-700 hover:text-white px-3" on:click={() => appWindow.unmaximize()}>
					<Icon src={VscChromeRestore} />
				</button>
			{:else}
				<button class="h-full hover:bg-neutral-700 hover:text-white px-3" on:click={() => appWindow.maximize()}>
					<Icon src={VscChromeMaximize} />
				</button>
			{/if}
		{/await}

		<button class="h-full hover:bg-red-600 hover:text-white px-3" on:click={() => appWindow.close()}>
			<Icon src={VscChromeClose} />
		</button>
	</div>
</div>
