import type { ServerLoad } from '@sveltejs/kit';
import { serverFetch } from '$lib/api/server';

export const load: ServerLoad = async ({ locals }) => {
	const token = locals.token;

	try {
		const { data, meta } = await serverFetch<unknown[]>('/payments', { token });

		return {
			payments: data ?? [],
			meta: meta ?? {}
		};
	} catch {
		return {
			payments: [],
			meta: {}
		};
	}
};
