export default class Context {
	public variables: Map<string, any>;
	public functions: Map<string, any>;

	constructor() {
		this.variables = new Map<string, any>();
		this.functions = new Map<string, any>();
	}
}
