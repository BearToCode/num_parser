import ansi from 'ansi-escape-sequences';
import type { Terminal } from 'xterm';
import { hexToRgb } from '@utils/colors';
import theme from '@utils/theme';

export default class termIO {
	private _terminalController: Terminal;
	private _typedChars: number;
	private _currentInput: string;
	private _timerId: NodeJS.Timeout;

	private _promptStr: string = '~ ';

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
		this._terminalController.onKey((e) => this.handleKey(e));
		this._terminalController.onResize((/*e*/) => this.debounceReflow(250));
	}

	prompt(): void {
		const colorHex = theme.colors.secondary['500'];
		const c = hexToRgb(colorHex);
		// @ts-ignore (for some reason rgb() is not included in @types/ansi-escape-sequences)
		this._terminalController.write(ansi.rgb(c.r, c.b, c.b) + this._promptStr + ansi.style.reset);
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
				// this.moveRight();
				break;
			case 'ArrowLeft':
				// this.moveLeft();
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

			if (this.cursorPosition().posX == 0) {
				this.setCursorPosition(
					this._terminalController.cols, // Right
					this.cursorPosition().posY
				);
				this._terminalController.write(ansi.erase.display(0));
			} else {
				this._terminalController.write('\b \b');
			}

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
		console.log(x, y);
		this._terminalController.write(ansi.cursor.position(y, x));
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
