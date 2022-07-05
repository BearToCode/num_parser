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
	class="h-8 bg-primary-700 toolbar text-primary-100 flex flex-row col-span-3
  items-center text-sm border-b-2 border-primary-800 z-10 justify-center relative"
>
	<div class="bg-secondary-700 rounded-lg px-3 text-xs text-white">
		{$currentLocation.pathname}
	</div>

	<div class="text-base flex h-full absolute right-0">
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
