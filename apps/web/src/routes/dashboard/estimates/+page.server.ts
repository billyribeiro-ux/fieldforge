import type { ServerLoad } from '@sveltejs/kit';
import { serverFetch } from '$lib/api/server';

export const load: ServerLoad = async ({ locals, url }) => {
	const token = locals.token;
	const status = url.searchParams.get('status') ?? '';

	try {
		const { data, meta } = await serverFetch<unknown[]>('/estimates', {
			token,
			params: { status }
		});

		return {
			estimates: data ?? [],
			meta: meta ?? {},
			filters: { status }
		};
	} catch (e) {
		return {
			estimates: [],
			meta: {},
			filters: { status },
			error: e instanceof Error ? e.message : 'Failed to load estimates'
		};
	}
};
