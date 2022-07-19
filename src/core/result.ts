export default class Result<Type> {
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
