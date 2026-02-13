import type { ServerLoad } from '@sveltejs/kit';
import { error } from '@sveltejs/kit';
import { serverFetch } from '$lib/api/server';

export const load: ServerLoad = async ({ locals, params }) => {
	const token = locals.token;
	const jobId = params.id;

	try {
		const [jobRes, timeEntriesRes, notesRes] = await Promise.allSettled([
			serverFetch<unknown>(`/jobs/${jobId}`, { token }),
			serverFetch<unknown[]>(`/jobs/${jobId}/time-entries`, { token }),
			serverFetch<unknown[]>(`/jobs/${jobId}/notes`, { token })
		]);

		const job = jobRes.status === 'fulfilled' ? jobRes.value.data : null;

		if (!job) {
			error(404, 'Job not found');
		}

		return {
			job,
			timeEntries: timeEntriesRes.status === 'fulfilled' ? timeEntriesRes.value.data : [],
			notes: notesRes.status === 'fulfilled' ? notesRes.value.data : []
		};
	} catch (e) {
		if (e && typeof e === 'object' && 'status' in e) throw e;
		return {
			job: null,
			timeEntries: [],
			notes: [],
			error: e instanceof Error ? e.message : 'Failed to load job'
		};
	}
};
