<script lang="ts">
	import Icon from 'svelte-icons-pack';
	import IoClose from 'svelte-icons-pack/io/IoClose';
	import FiChevronUp from 'svelte-icons-pack/fi/FiChevronUp';
	import FiChevronDown from 'svelte-icons-pack/fi/FiChevronDown';
	import BiMinus from 'svelte-icons-pack/bi/BiMinus';

	import { createEventDispatcher } from 'svelte';
	export let icon: any;

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

<div class="w-full h-full space-y-2 flex flex-col relative">
	<div class="w-full flex justify-between text-lg items-center text-neutral-400 bg-primary-600">
		<div class="text-2xl p-1 pb-2 text-white relative">
			<Icon color="currentcolor" src={icon} />
			<div class="absolute bottom-1 left-1 right-1 h-0.5 rounded bg-white" />
		</div>
		<div class="flex justify-end pr-2">
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
	</div>
	<div class="px-2">
		<slot />
	</div>
</div>
