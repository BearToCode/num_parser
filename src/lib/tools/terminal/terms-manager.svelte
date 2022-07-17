<script lang="ts">
	import Icon from 'svelte-icons-pack';
	import BiSolidTerminal from 'svelte-icons-pack/bi/BiSolidTerminal';
	import BiPlus from 'svelte-icons-pack/bi/BiPlus';
	import BiMinus from 'svelte-icons-pack/bi/BiMinus';

	import Terminal from './terminal.svelte';

	let active: number = 0;
	let next: number = 1;

	let term_windows: { id: number; wind: any }[] = [{ id: 0, wind: Terminal }];
</script>

<div class="w-full h-full relative flex">
	<!-- <Terminal /> -->
	<div class="flex-grow p-2">
		<div class="w-full h-full relative">
			{#each term_windows as win}
				<svelte:component this={win.wind} hidden={win.id != active} />
			{/each}
		</div>
	</div>
	<div class="flex py-1 flex-col border-l border-primary-550">
		<div class="flex-grow overflow-y-scroll scrollbar-width-none">
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

		<div class="flex flex-col px-1.5 pt-1 border-t border-primary-550">
			<div
				class="flex justify-center text-lg text-primary-400 hover:text-white cursor-pointer"
				on:click={() => {
					for (let i = 0; i < term_windows.length; i++) {
						if (active == term_windows[i].id) {
							term_windows.splice(i, 1);
							active = term_windows[Math.max(0, i - 1)].id;
							break;
						}
					}

					// Trigger update
					term_windows = term_windows;
				}}
			>
				<Icon color="currentcolor" src={BiMinus} />
			</div>

			<div
				class="flex justify-center text-lg text-primary-400 hover:text-white cursor-pointer"
				on:click={() => {
					term_windows.push({ id: next, wind: Terminal });
					next += 1;

					// Trigger update
					term_windows = term_windows;
				}}
			>
				<Icon color="currentcolor" src={BiPlus} />
			</div>
		</div>
	</div>
</div>

<style>
	.scrollbar-width-none::-webkit-scrollbar {
		display: none; /* Safari and Chrome */
	}
</style>
