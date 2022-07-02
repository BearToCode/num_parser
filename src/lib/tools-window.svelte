<script lang="ts">
	import Icon from 'svelte-icons-pack';
	import IoClose from 'svelte-icons-pack/io/IoClose';

	import { createEventDispatcher } from 'svelte';

	const dispatch = createEventDispatcher();

	function close() {
		dispatch('close');
	}
</script>

<div class="w-full h-full relative p-2">
	<div class="w-full justify-end flex items-center text-neutral-400">
		<button on:click={close}>
			<Icon color="currentcolor" src={IoClose} />
		</button>
	</div>
</div>
