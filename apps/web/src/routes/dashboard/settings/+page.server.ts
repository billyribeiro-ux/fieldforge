import type { ServerLoad } from '@sveltejs/kit';
import { serverFetch } from '$lib/api/server';

export const load: ServerLoad = async ({ locals }) => {
	const token = locals.token;

	try {
		const [teamRes, membersRes] = await Promise.allSettled([
			serverFetch<unknown>('/team', { token }),
			serverFetch<unknown[]>('/team/members', { token })
		]);

		return {
			team: teamRes.status === 'fulfilled' ? teamRes.value.data : null,
			members: membersRes.status === 'fulfilled' ? membersRes.value.data : []
		};
	} catch {
		return {
			team: null,
			members: []
		};
	}
};
