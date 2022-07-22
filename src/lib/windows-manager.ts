import { Writable, writable } from 'svelte/store';

export type Window = { name: string; icon: any; component: any };
export type WindowState = 'visible' | 'hidden';

// Tools windows
import Terminal from './tools/terminal/terms-manager.svelte';
import BiTerminal from 'svelte-icons-pack/bi/BiTerminal';

export let toolsWindows: Window[] = [{ name: 'Terminal', icon: BiTerminal, component: Terminal }];

const DEFAULT_SIZE = 30;

export const currentToolsWindow: Writable<Window> = writable(toolsWindows[0]);
export const toolsWindowSize: Writable<number> = writable(DEFAULT_SIZE);
export const toolsWindowState: Writable<WindowState> = writable('hidden');

let currentState: WindowState;

toolsWindowState.subscribe((value) => {
	currentState = value;
});

export const toggleToolsWindow = () => {
	if (currentState == 'hidden') {
		toolsWindowState.set('visible');
	} else {
		toolsWindowState.set('hidden');
	}
};
