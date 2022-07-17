import ansi from 'ansi-escape-sequences';
import type { Terminal } from 'xterm';
import Context from '@core/context';
import { hexToRgb } from '@utils/colors';
import theme from '@utils/theme';
import { SendDeclaration, Result, SendEvaluation } from '@core/api';

export default class termIO {
	private _terminalController: Terminal;
	private _promptStr: string = '> ';

	private _context: Context;

	constructor(term: Terminal) {
		this._terminalController = term;
		this._context = new Context();
		this.write(ansi.cursor.hide);
	}

	async prompt(): Promise<void> {
		const colorHex = theme.colors.secondary['500'];
		const c = hexToRgb(colorHex);
		// @ts-ignore (for some reason rgb() is not included in @types/ansi-escape-sequences)
		await this.write(ansi.rgb(c.r, c.b, c.b) + this._promptStr + ansi.style.reset);
	}

	public async execute(command: string) {
		if (command == 'clear') {
			this._terminalController.reset();
			return;
		}

		await this.prompt();
		await this.write(command);
		await this.write('\r\n');

		if (command.indexOf('=') == -1) {
			let out = await SendEvaluation(command, this._context);
			let result: number | null = out.expect(this.write.bind(this));
			if (result) {
				await this.write(`${result}`);
			}
		} else {
		}

		this._terminalController.write('\r\n');
	}

	async write(s: string): Promise<void> {
		return new Promise((resolve) => {
			this._terminalController.write(s, () => resolve());
		});
	}
}
