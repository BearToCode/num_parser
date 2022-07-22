<script lang="ts">
	import { Pane, Splitpanes } from 'svelte-splitpanes';
	// Neon theme
	import '@styles/split-themes.scss';

	import ToolsWindow from './tools-window.svelte';
	import {
		toggleToolsWindow,
		toolsWindowSize,
		currentToolsWindow,
		toolsWindowState,
		WindowState,
		Window,
	} from './windows-manager';

	let unmaximizedSize: number;

	// Subscriptions
	let size: number;
	let status: WindowState;
	let window: Window;

	toolsWindowSize.subscribe((value) => {
		size = value;
	});

	toolsWindowState.subscribe((value) => {
		status = value;
	});

	currentToolsWindow.subscribe((value) => {
		window = value;
	});

	// Size methods
	const maximizeToolsWindow = () => {
		unmaximizedSize = size;
		toolsWindowSize.set(100);
	};

	const restoreToolsWindow = () => {
		toolsWindowSize.set(unmaximizedSize);
	};

	const minimizeToolsWindow = () => {
		toolsWindowSize.set(0);
	};
</script>

<Splitpanes theme="neon-theme" dblClickSplitter={false} class="overflow-hidden">
	<Pane size={30} snapSize={5} maxSize={50} class="bg-primary-600" />
	<Pane class="bg-primary-700">
		<Splitpanes theme="neon-theme" dblClickSplitter={false} horizontal={true}>
			<Pane snapSize={10} size={100 - size}>
				<slot />
			</Pane>

			{#if status == 'visible'}
				<Pane snapSize={10} bind:size class="flex flex-col">
					<ToolsWindow
						icon={window.icon}
						isMaximized={size == 100}
						on:minimize={minimizeToolsWindow}
						on:restore={restoreToolsWindow}
						on:maximize={maximizeToolsWindow}
						on:close={toggleToolsWindow}
						tool={window.component}
					/>
				</Pane>
			{/if}
		</Splitpanes>
	</Pane>
</Splitpanes>
