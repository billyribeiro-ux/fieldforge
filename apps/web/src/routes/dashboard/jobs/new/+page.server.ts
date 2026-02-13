import type { ServerLoad } from '@sveltejs/kit';
import { serverFetch } from '$lib/api/server';

export const load: ServerLoad = async ({ locals }) => {
	const token = locals.token;

	try {
		const [customersRes, membersRes] = await Promise.allSettled([
			serverFetch<unknown[]>('/customers?limit=100', { token }),
			serverFetch<unknown[]>('/teams/members', { token })
		]);

		return {
			customers: customersRes.status === 'fulfilled' ? (customersRes.value.data ?? []) : [],
			members: membersRes.status === 'fulfilled' ? (membersRes.value.data ?? []) : []
		};
	} catch {
		return {
			customers: [],
			members: []
		};
	}
};
