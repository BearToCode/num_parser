import ansi from 'ansi-escape-sequences';
import type { Terminal } from 'xterm';

export default class termIO {
	private _terminalController: Terminal;
	private _typedChars: number;
	private _currentInput: string;
	private _timerId: NodeJS.Timeout;

	private _promptStr: string = '~ ';
	private _promptLength: number;

	private _currentLine: number;

	constructor(term: Terminal) {
		this._terminalController = term;
		this._typedChars = 0;
		this._currentLine = 1;
		this._currentInput = '';
		this.initializeTerminal();
	}

	initializeTerminal(): void {
		this.prompt();
		this._promptLength = this._terminalController.buffer.active.cursorX;
		this._terminalController.onKey((e) => this.handleKey(e));
		this._terminalController.onResize((e) => this.debounceReflow(250));
	}

	prompt(): void {
		this._terminalController.write(ansi.style.red + this._promptStr + ansi.style.reset);
	}

	handleKey(e: { key: string; domEvent: KeyboardEvent }): void {
		const keyStr = e.domEvent.key;
		switch (keyStr) {
			case 'Enter':
				break;
			case 'Delete':
			case 'Backspace':
				this.del();
			case 'ArrowRight':
				break;
			case 'ArrowLeft':
				break;
			case 'ArrowUp':
				break;
			case 'ArrowDown':
				break;
			case 'Insert':
				break;
			case 'Home':
				break;
			case 'PageUp':
				break;
			case 'PageDown':
				break;
			case 'End':
				break;
			case 'PageDown':
				break;
			case 'Tab':
				break;
			default:
				this.write(keyStr);
		}
	}

	write(s: string): void {
		this._typedChars += s.length;
		this._currentInput += s;

		this._terminalController.write(s);
	}

	del(n: number = 1) {
		for (let i = 0; i < n; i++) {
			if (this._typedChars == 0) {
				return;
			}

			if (this.cursorPosition().posX == 1) {
				this.setCursorPosition(
					this._terminalController.cols, // Right
					this.cursorPosition().posY - 1 // Up one
				);
			}

			console.log('Ciao');

			this._terminalController.write('\b \b');
			this._typedChars -= 1;
		}
	}

	debounceReflow(delay: number): void {
		// Implements a debouncing technique to prevent too many calls
		if (this._timerId) {
			clearTimeout(this._timerId);
		}
		let that = this;

		this._timerId = setTimeout(function () {
			that.reflowInput();
		}, delay);
	}

	reflowInput(): void {
		console.log('Hello');

		this.deleteInput();

		// Rewrite data
		this.prompt();
		this._terminalController.write(this._currentInput);
	}

	cursorPosition(): { posX: number; posY: number } {
		return {
			posX: this._terminalController.buffer.active.cursorX,
			posY: this._terminalController.buffer.active.cursorY,
		};
	}

	setCursorPosition(x: number, y: number): void {
		this._terminalController.write(ansi.cursor.position(x, y));
	}

	deleteInput(): void {
		this.setCursorPosition(1, this.cursorPosition().posY);

		for (let i = this.cursorPosition().posX; i >= this._currentLine; i--) {
			// Clear line
			this._terminalController.write(ansi.erase.display(0));

			// Set the cursor up one line
			this._terminalController.write(ansi.cursor.up());
		}
	}
}
