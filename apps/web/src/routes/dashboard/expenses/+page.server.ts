import type { ServerLoad } from '@sveltejs/kit';
import { serverFetch } from '$lib/api/server';

export const load: ServerLoad = async ({ locals }) => {
	const token = locals.token;

	try {
		const { data, meta } = await serverFetch<unknown[]>('/expenses', { token });

		return {
			expenses: data ?? [],
			meta: meta ?? {}
		};
	} catch {
		return {
			expenses: [],
			meta: {}
		};
	}
};
