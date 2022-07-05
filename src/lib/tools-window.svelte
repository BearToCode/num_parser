<script lang="ts">
	import Icon from 'svelte-icons-pack';
	import IoClose from 'svelte-icons-pack/io/IoClose';
	import FiChevronUp from 'svelte-icons-pack/fi/FiChevronUp';
	import FiChevronDown from 'svelte-icons-pack/fi/FiChevronDown';
	import BiMinus from 'svelte-icons-pack/bi/BiMinus';

	import { createEventDispatcher } from 'svelte';

	const dispatch = createEventDispatcher();

	function close() {
		dispatch('close');
	}

	export let isMaximized: boolean;

	function maximize() {
		dispatch('maximize');
	}

	function restore() {
		dispatch('restore');
	}

	function minimize() {
		dispatch('minimize');
	}
</script>

<div class="w-full h-full flex flex-col relative p-2">
	<div class="w-full justify-end text-lg flex items-center text-neutral-400">
		<button class="p-0.5 hover:bg-primary-600 rounded" on:click={minimize}>
			<Icon color="currentcolor" src={BiMinus} />
		</button>
		{#if isMaximized}
			<button class="p-0.5 hover:bg-primary-600 rounded" on:click={restore}>
				<Icon color="currentcolor" src={FiChevronDown} />
			</button>
		{:else}
			<button class="p-0.5 hover:bg-primary-600 rounded" on:click={maximize}>
				<Icon color="currentcolor" src={FiChevronUp} />
			</button>
		{/if}
		<button class="p-0.5 hover:bg-primary-600 rounded" on:click={close}>
			<Icon color="currentcolor" src={IoClose} />
		</button>
	</div>
	<slot />
</div>
