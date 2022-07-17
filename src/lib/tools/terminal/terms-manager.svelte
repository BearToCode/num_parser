<script lang="ts">
	import Icon from 'svelte-icons-pack';
	import BiSolidTerminal from 'svelte-icons-pack/bi/BiSolidTerminal';

	import Terminal from './terminal.svelte';

	let active: number = 0;

	let term_windows: { id: number; wind: any }[] = [
		{ id: 0, wind: Terminal },
		{ id: 1, wind: Terminal },
	];
</script>

<div class="w-full h-full relative flex">
	<!-- <Terminal /> -->
	<div class="flex-grow p-2">
		<div class="w-full h-full relative">
			{#each term_windows as win}
				{#if win.id == active}
					<svelte:component this={win.wind} />
				{/if}
			{/each}
		</div>
	</div>
	<div class="flex py-1 flex-col border-l border-primary-550">
		{#each term_windows as win}
			{#if win.id == active}
				<div class="w-full cursor-pointer text-xl px-1 text-primary-100 border-l border-primary-100">
					<Icon color="currentcolor" src={BiSolidTerminal} />
				</div>
			{:else}
				<div
					class="w-full cursor-pointer text-xl px-1 text-primary-500 border-l border-transparent"
					on:click={() => (active = win.id)}
				>
					<Icon color="currentcolor" src={BiSolidTerminal} />
				</div>
			{/if}
		{/each}
	</div>
</div>
