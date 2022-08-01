import ansi from 'ansi-escape-sequences';
import type { Terminal } from 'xterm';
import type Context from '@core/context';
import { hexToRgb } from '@utils/colors';
import theme from '@utils/theme';
import { CreateEmptyContext, EvaluateWithMutableContext } from '@core/api';

export default class termIO {
	private _terminalController: Terminal;
	private _promptStr: string = '> ';

	private _context: Context;

	async init(term: Terminal) {
		this._terminalController = term;
		this._context = await CreateEmptyContext();
		this.write(ansi.cursor.hide);
	}

	async prompt(): Promise<void> {
		const colorHex = theme.colors.secondary['500'];
		const c = hexToRgb(colorHex);
		// @ts-ignore (for some reason rgb() is not included in @types/ansi-escape-sequences)
		await this.write(ansi.rgb(c.r, c.b, c.b) + this._promptStr + ansi.style.reset);
	}

	public async execute(command: string) {
		if (command == 'clear') return this._terminalController.reset();

		if (command == '') {
			await this.prompt();
			return this.write('\r\n');
		}

		await this.prompt();
		await this.write(command);
		await this.write('\r\n');

		await EvaluateWithMutableContext(command, this._context).then((r) => {
			r[0].log(this.logOk.bind(this), this.logErr.bind(this));
			this._context = r[1];
		});
	}

	async write(s: string): Promise<void> {
		return new Promise((resolve) => {
			this._terminalController.write(s, () => resolve());
		});
	}

	async logOk(s: string): Promise<void> {
		await this.write(ansi.style.white);
		await this.write(s);
		await this.write(ansi.style.reset);
		await this.write('\r\n');
	}

	async logErr(s: string): Promise<void> {
		await this.write(ansi.style.white);
		await this.write(ansi.style['bg-red']);
		await this.write(s);
		await this.write(ansi.style.reset);
		await this.write('\r\n');
	}
}
