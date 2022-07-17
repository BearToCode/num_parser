<script lang="ts">
	import { createEventDispatcher } from 'svelte/internal';

	const dispatch = createEventDispatcher();

	export const focus = () => {
		input.focus();
	};

	let executedCommands: string[] = [];
	let executedCommandsIndex = -1;
	let tempInput: string;

	let prompt = '~ ';
	let caretPos = 0;

	let isFocused = false;

	function dispatchData() {
		dispatch('command', { command: input.value });
	}

	function setCaretPosition(translation: number) {
		caret.style.transform = 'translateX( ' + translation + 'px )';
	}

	function updateCaretPosition() {
		setCaretPosition(renderer.clientWidth);
	}

	function updateRenderer() {
		renderer.innerText = input.value.slice(0, caretPos).replaceAll(' ', '-');
		updateCaretPosition();
	}

	let input: HTMLInputElement;
	let renderer: HTMLElement;
	let caret: HTMLElement;
</script>

<div
	class="absolute left-0 right-0 bottom-0 h-min flex py-1 bg-primary-700 z-10 text-md"
	on:click={() => {
		caretPos = input.selectionStart;
		updateRenderer();
	}}
>
	<p class="text-secondary-500 w-5">{prompt}</p>
	<p
		class="absolute left-5 max-w-full text-transparent select-none inline-block align-middle top-1 bottom-1 "
		bind:this={renderer}
	/>
	<input
		bind:this={input}
		spellcheck="false"
		class="relative flex-grow bg-transparent caret-transparent outline-none text-white bg-primary-700"
		on:focus={() => {
			isFocused = true;
		}}
		on:blur={() => {
			isFocused = false;
		}}
		on:keydown={(e) => {
			window.setTimeout(() => {
				switch (e.key) {
					case 'Enter':
						executedCommands = [input.value].concat(executedCommands);
						dispatchData();
						input.value = '';
						updateRenderer();
						break;
					case '(':
						caretPos = input.selectionStart;
						updateRenderer();
						input.value += ')';
						input.selectionStart = caretPos;
						input.selectionEnd = caretPos;
						break;
					case 'ArrowUp':
						e.preventDefault();
						if (executedCommandsIndex + 1 < executedCommands.length) {
							if (executedCommandsIndex == -1) {
								tempInput = input.value;
							}
							executedCommandsIndex += 1;
							input.value = executedCommands[executedCommandsIndex];
							updateRenderer();
						}
						break;
					case 'ArrowDown':
						e.preventDefault();
						if (executedCommandsIndex - 1 == -1) {
							executedCommandsIndex -= 1;
							input.value = tempInput;
							updateRenderer();
						} else if (executedCommandsIndex - 1 >= 0) {
							executedCommandsIndex -= 1;
							input.value = executedCommands[executedCommandsIndex];
							updateRenderer();
						}
						break;
					default:
						executedCommandsIndex = -1;
						caretPos = input.selectionStart;
						updateRenderer();
				}
			}, 1);
		}}
	/>
	<div
		bind:this={caret}
		class={`absolute left-5 top-1 bottom-1 w-3 border border-primary-100 ${isFocused && 'backdrop-invert'}`}
		id="caret"
	/>
</div>

<style lang="postcss">
	::selection {
		@apply bg-primary-50 text-primary-700;
	}
</style>
