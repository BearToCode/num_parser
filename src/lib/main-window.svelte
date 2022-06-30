<script lang="ts">
	import { Pane, Splitpanes } from 'svelte-splitpanes';

	let toolbarOpen = false;
</script>

<Splitpanes dblClickSplitter={false} horizontal={true}>
	<Pane snapSize="10">
		<slot />
		<button class="w-12 h-12 bg-red-400" on:click={() => (toolbarOpen = !toolbarOpen)} />
	</Pane>

	{#if toolbarOpen}
		<Pane snapSize="10" size="30" />
	{/if}
</Splitpanes>
