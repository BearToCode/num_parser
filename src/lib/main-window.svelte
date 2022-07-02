<script lang="ts">
	import { Pane, Splitpanes } from 'svelte-splitpanes';
	// Neon theme
	import _ from '../styles/split-themes.scss';

	import ToolsWindow from './tools-window.svelte';

	let toolsWindowOpen = false;

	export const toggleToolsWindow = () => {
		toolsWindowOpen = !toolsWindowOpen;
	};
</script>

<Splitpanes theme="neon-theme" dblClickSplitter={false} horizontal={true} class="bg-primary-200">
	<Pane snapSize={10}>
		<slot />
	</Pane>

	{#if toolsWindowOpen}
		<Pane snapSize={10} size={30}>
			<ToolsWindow on:close={toggleToolsWindow} />
		</Pane>
	{/if}
</Splitpanes>
