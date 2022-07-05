<script lang="ts">
	import { Pane, Splitpanes } from 'svelte-splitpanes';
	// Neon theme
	import _ from '../styles/split-themes.scss';

	import ToolsWindow from './tools-window.svelte';

	let toolsWindowOpen = false;
	let currentToolWindow: { name: string; icon: any; component: any };
	let toolsWindowSize = 30;
	let unmaximizedSize = toolsWindowSize;

	export const toggleToolsWindow = () => {
		toolsWindowOpen = !toolsWindowOpen;
	};

	export const setToolsWindow = (window: { name: string; icon: any; component: any }) => {
		currentToolWindow = window;
	};

	const maximizeToolsWindow = () => {
		unmaximizedSize = toolsWindowSize;
		toolsWindowSize = 100;
	};

	const restoreToolsWindow = () => {
		toolsWindowSize = unmaximizedSize;
	};

	const minimizeToolsWindow = () => {
		toolsWindowSize = 0;
	};
</script>

<Splitpanes theme="neon-theme" dblClickSplitter={false} class="overflow-hidden">
	<Pane size={30} snapSize={5} maxSize={50} class="bg-primary-600" />
	<Pane class="bg-primary-700">
		<Splitpanes theme="neon-theme" dblClickSplitter={false} horizontal={true}>
			<Pane snapSize={10} size={toolsWindowOpen ? 100 - toolsWindowSize : 100}>
				<slot />
			</Pane>

			{#if toolsWindowOpen}
				<Pane snapSize={10} bind:size={toolsWindowSize} class="flex flex-col">
					<ToolsWindow
						icon={currentToolWindow.icon}
						isMaximized={toolsWindowSize == 100}
						on:minimize={minimizeToolsWindow}
						on:restore={restoreToolsWindow}
						on:maximize={maximizeToolsWindow}
						on:close={toggleToolsWindow}
					>
						<svelte:component this={currentToolWindow.component} />
					</ToolsWindow>
				</Pane>
			{/if}
		</Splitpanes>
	</Pane>
</Splitpanes>
