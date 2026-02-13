import type { ServerLoad } from '@sveltejs/kit';
import { serverFetch } from '$lib/api/server';

export const load: ServerLoad = async ({ locals }) => {
	const token = locals.token;

	try {
		const { data } = await serverFetch<unknown[]>('/reviews', { token });

		return {
			reviews: data ?? []
		};
	} catch {
		return {
			reviews: []
		};
	}
};
