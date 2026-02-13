import type { ServerLoad } from '@sveltejs/kit';
import { serverFetch } from '$lib/api/server';

export const load: ServerLoad = async ({ locals }) => {
	const token = locals.token;

	try {
		const [licensesRes, policiesRes] = await Promise.allSettled([
			serverFetch<unknown[]>('/licenses', { token }),
			serverFetch<unknown[]>('/insurance-policies', { token })
		]);

		return {
			licenses: licensesRes.status === 'fulfilled' ? (licensesRes.value.data ?? []) : [],
			policies: policiesRes.status === 'fulfilled' ? (policiesRes.value.data ?? []) : []
		};
	} catch {
		return {
			licenses: [],
			policies: []
		};
	}
};
