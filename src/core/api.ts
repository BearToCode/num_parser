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

	public expect(logMethod: (str: string) => any): Type | null {
		if (this.isValid()) {
			return this.content as Type;
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

export async function SendEvaluation(input: string, context: Context): Promise<Result<number>> {
	let out = await invoke('evaluate_expression', { input: input, context: context });
	console.log(out);
	return new Result<number>('Evaluation error');
}
