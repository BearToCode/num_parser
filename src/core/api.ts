import type Context from './context';
import { invoke } from '@tauri-apps/api';
import Result from './result';

export async function EvaluateWithStaticContext(input: string, context: Context): Promise<Result<string>> {
	return invoke('evaluate_with_static_context', { input: input, context: context })
		.then((r) => {
			return new Result<string>(r as string, 'Ok');
		})
		.catch((e) => {
			return new Result<string>(e as string, 'Err');
		});
}

export async function EvaluateWithMutableContext(input: string, context: Context): Promise<[Result<string>, Context]> {
	return invoke('evaluate_with_mutable_context', { input: input, context: context })
		.then((r) => {
			console.log(r);
			return [new Result<string>(r[0] as string, 'Ok'), r[1]] as [Result<string>, Context];
		})
		.catch((e) => {
			return [new Result<string>(e as string, 'Err'), context];
		});
}
