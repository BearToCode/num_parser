import type Context from './context';
import ansi from 'ansi-escape-sequences';
import { invoke } from '@tauri-apps/api';

export class Result<Type> {
	public content: Type | string;

	constructor(out: Type | string) {
		this.content = out;
	}

	public isValid(): boolean {
		return !(typeof this.content === 'string');
	}

	public expect(logMethod: (str: string) => any): Context | null {
		if (this.isValid()) {
			return this.content;
		} else {
			logMethod(ansi.style.red);
			logMethod(this.content as string);
			logMethod(ansi.style.reset);
		}
	}
}

export function SendDeclaration(input: string, context: Context): Result<Context> {
	let out = invoke('add_declaration', { context: context });
	console.log(out);
	return new Result<Context>('Declaration error');
}

export function SendEvaluation(input: string, context: Context): Result<number> {
	let out = invoke('evaluate_expression', { input: input, context: context });
	console.log(out);
	return new Result<number>('Evaluation error');
}
