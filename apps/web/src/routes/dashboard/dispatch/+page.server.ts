import type { ServerLoad } from '@sveltejs/kit';
import { serverFetch } from '$lib/api/server';

export const load: ServerLoad = async ({ locals }) => {
	const token = locals.token;

	try {
		const [jobsRes, membersRes] = await Promise.allSettled([
			serverFetch<unknown[]>('/jobs?status=scheduled,en_route,in_progress', { token }),
			serverFetch<unknown[]>('/teams/members', { token })
		]);

		return {
			jobs: jobsRes.status === 'fulfilled' ? (jobsRes.value.data ?? []) : [],
			members: membersRes.status === 'fulfilled' ? (membersRes.value.data ?? []) : []
		};
	} catch {
		return {
			jobs: [],
			members: []
		};
	}
};
