import ansi from 'ansi-escape-sequences';
import type { Terminal } from 'xterm';

export default class termIO {
	private _terminalController: Terminal;
	private _typedChars: number;
	private _currentInput: string;
	private _timerId: NodeJS.Timeout;

	private _promptStr: string = '\u001b[36mnumerus> \u001b';
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
		this._terminalController.onResize((e) => this.debounceReflow(500));
	}

	prompt(): void {
		this._terminalController.write(this._promptStr);
	}

	handleKey(e: { key: string; domEvent: KeyboardEvent }): void {
		const keyStr = e.domEvent.key;
		if (keyStr === 'Enter') {
		} else if (keyStr === 'Backspace') {
		} else if (['ArrowRight', 'ArrowLeft', 'ArrowUp', 'ArrowDown'].indexOf(keyStr) != -1) {
		} else {
			this.write(keyStr);
		}
	}

	write(s: string): void {
		this._typedChars += s.length;
		this._currentInput += s;

		this._terminalController.write(s);
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
