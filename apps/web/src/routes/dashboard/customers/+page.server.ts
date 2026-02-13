import type { ServerLoad } from '@sveltejs/kit';
import { serverFetch } from '$lib/api/server';

export const load: ServerLoad = async ({ locals, url }) => {
	const token = locals.token;
	const search = url.searchParams.get('q') ?? '';

	try {
		const { data, meta } = await serverFetch<unknown[]>('/customers', {
			token,
			params: { q: search }
		});

		return {
			customers: data ?? [],
			meta: meta ?? {},
			filters: { search }
		};
	} catch (e) {
		return {
			customers: [],
			meta: {},
			filters: { search },
			error: e instanceof Error ? e.message : 'Failed to load customers'
		};
	}
};
