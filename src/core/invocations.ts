import Context from './context';

export class Result {
	public content: Context | string;

	constructor(out: Context | string) {
		this.content = out;
	}

	public isValid(): boolean {
		return typeof this.content == typeof Context;
	}

	public expect(logMethod: (str: string) => any): Context | null {
		if (this.isValid()) {
			return this.content;
		} else {
			logMethod(this.content as string);
		}
	}
}

export function SendDeclaration(input: string, context: Context): Result {
	return new Result('Declaration error');
}

export function SendEvaluation(input: string, context: Context): Result {
	return new Result('Evaluation error');
}
