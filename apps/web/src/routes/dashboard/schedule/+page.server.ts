import type { ServerLoad } from '@sveltejs/kit';
import { serverFetch } from '$lib/api/server';

export const load: ServerLoad = async ({ locals, url }) => {
	const token = locals.token;
	const week = url.searchParams.get('week') ?? '';

	try {
		const { data, meta } = await serverFetch<unknown[]>('/jobs', {
			token,
			params: { status: 'scheduled,in_progress,en_route', week }
		});

		return {
			scheduledJobs: data ?? [],
			meta: meta ?? {},
			filters: { week }
		};
	} catch {
		return {
			scheduledJobs: [],
			meta: {},
			filters: { week }
		};
	}
};
