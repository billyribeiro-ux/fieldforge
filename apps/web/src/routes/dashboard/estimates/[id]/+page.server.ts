import type { ServerLoad } from '@sveltejs/kit';
import { error } from '@sveltejs/kit';
import { serverFetch } from '$lib/api/server';

export const load: ServerLoad = async ({ locals, params }) => {
	const token = locals.token;
	const estimateId = params.id;

	try {
		const { data } = await serverFetch<unknown>(`/estimates/${estimateId}`, { token });

		if (!data) {
			error(404, 'Estimate not found');
		}

		return { estimate: data };
	} catch (e) {
		if (e && typeof e === 'object' && 'status' in e) throw e;
		return {
			estimate: null,
			error: e instanceof Error ? e.message : 'Failed to load estimate'
		};
	}
};
