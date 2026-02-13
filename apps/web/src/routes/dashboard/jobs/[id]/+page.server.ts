import type { ServerLoad, Actions } from '@sveltejs/kit';
import { error, fail } from '@sveltejs/kit';
import { serverFetch, serverPatch, serverPost } from '$lib/api/server';

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

export const actions: Actions = {
	updateStatus: async ({ request, locals, params }) => {
		const token = locals.token;
		if (!token) return fail(401, { error: 'Not authenticated' });

		const formData = await request.formData();
		const status = formData.get('status') as string;

		if (!status) return fail(400, { error: 'Status is required' });

		try {
			await serverPatch(`/jobs/${params.id}/status`, { status }, { token });
			return { success: true };
		} catch (e) {
			return fail(500, { error: e instanceof Error ? e.message : 'Failed to update status' });
		}
	},

	addNote: async ({ request, locals, params }) => {
		const token = locals.token;
		if (!token) return fail(401, { error: 'Not authenticated' });

		const formData = await request.formData();
		const content = formData.get('content') as string;
		const is_internal = formData.get('is_internal') === 'true';

		if (!content?.trim()) return fail(400, { error: 'Note content is required' });

		try {
			await serverPost(`/jobs/${params.id}/notes`, {
				content: content.trim(),
				is_internal
			}, { token });
			return { success: true };
		} catch (e) {
			return fail(500, { error: e instanceof Error ? e.message : 'Failed to add note' });
		}
	}
};
