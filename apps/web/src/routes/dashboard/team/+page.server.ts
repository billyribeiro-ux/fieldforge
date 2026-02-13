import type { ServerLoad } from '@sveltejs/kit';
import { serverFetch } from '$lib/api/server';

export const load: ServerLoad = async ({ locals }) => {
	const token = locals.token;

	try {
		const [usersRes] = await Promise.allSettled([
			serverFetch<unknown[]>('/teams/members', { token })
		]);

		return {
			members: usersRes.status === 'fulfilled' ? (usersRes.value.data ?? []) : []
		};
	} catch {
		return {
			members: []
		};
	}
};
