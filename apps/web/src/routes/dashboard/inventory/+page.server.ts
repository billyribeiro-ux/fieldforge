import type { ServerLoad } from '@sveltejs/kit';
import { serverFetch } from '$lib/api/server';

export const load: ServerLoad = async ({ locals, url }) => {
	const token = locals.token;
	const category = url.searchParams.get('category') ?? '';
	const search = url.searchParams.get('q') ?? '';

	try {
		const { data, meta } = await serverFetch<unknown[]>('/inventory', {
			token,
			params: { category, q: search }
		});

		return {
			items: data ?? [],
			meta: meta ?? {},
			filters: { category, search }
		};
	} catch {
		return {
			items: [],
			meta: {},
			filters: { category, search }
		};
	}
};
