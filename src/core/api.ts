import type Context from './context';
import { invoke } from '@tauri-apps/api';

export class Result<Type> {
	private _content: Type | string;
	private _type: 'Ok' | 'Err';

	constructor(out: Type | string, type: 'Ok' | 'Err') {
		this._content = out;
		this._type = type;
	}

	public isValid(): boolean {
		return this._type == 'Ok';
	}

	public log(OkLogMethod: (str: string) => any, ErrLogMethod: (str: string) => any): void {
		if (this.isValid()) {
			OkLogMethod(this._content.toString());
		} else {
			ErrLogMethod(this._content as string);
		}
	}

	public extract(): Type | string {
		return this._content;
	}
}

export async function SendDeclaration(input: string, context: Context): Promise<Result<Context>> {
	return invoke('add_declaration', { input: input, context: context })
		.then((r) => {
			return new Result<Context>(r as Context, 'Ok');
		})
		.catch((e) => {
			return new Result<Context>(e as string, 'Err');
		});
}

export async function SendEvaluation(input: string, context: Context): Promise<Result<number>> {
	return invoke('evaluate_expression', { input: input, context: context })
		.then((r) => {
			return new Result<number>(r as number, 'Ok');
		})
		.catch((e) => {
			return new Result<number>(e as string, 'Err');
		});
}
