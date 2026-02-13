import type { ServerLoad } from '@sveltejs/kit';
import { serverFetch } from '$lib/api/server';

export const load: ServerLoad = async ({ locals, url }) => {
	const token = locals.token;
	const status = url.searchParams.get('status') ?? '';

	try {
		const { data, meta } = await serverFetch<unknown[]>('/invoices', {
			token,
			params: { status }
		});

		return {
			invoices: data ?? [],
			meta: meta ?? {},
			filters: { status }
		};
	} catch (e) {
		return {
			invoices: [],
			meta: {},
			filters: { status },
			error: e instanceof Error ? e.message : 'Failed to load invoices'
		};
	}
};
