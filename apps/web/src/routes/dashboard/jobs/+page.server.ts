import type { ServerLoad } from '@sveltejs/kit';
import { serverFetch } from '$lib/api/server';

export const load: ServerLoad = async ({ locals, url }) => {
	const token = locals.token;
	const status = url.searchParams.get('status') ?? '';
	const search = url.searchParams.get('q') ?? '';

	try {
		const { data, meta } = await serverFetch<unknown[]>('/jobs', {
			token,
			params: { status, q: search }
		});

		return {
			jobs: data ?? [],
			meta: meta ?? {},
			filters: { status, search }
		};
	} catch (e) {
		return {
			jobs: [],
			meta: {},
			filters: { status, search },
			error: e instanceof Error ? e.message : 'Failed to load jobs'
		};
	}
};
