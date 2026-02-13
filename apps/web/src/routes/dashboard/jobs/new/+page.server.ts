import type { ServerLoad, Actions } from '@sveltejs/kit';
import { fail, redirect } from '@sveltejs/kit';
import { serverFetch, serverPost } from '$lib/api/server';

export const load: ServerLoad = async ({ locals }) => {
	const token = locals.token;

	try {
		const [customersRes, membersRes] = await Promise.allSettled([
			serverFetch<unknown[]>('/customers', { token, params: { limit: '100' } }),
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

export const actions: Actions = {
	create: async ({ request, locals }) => {
		const token = locals.token;
		if (!token) return fail(401, { error: 'Not authenticated' });

		const formData = await request.formData();
		const customer_id = formData.get('customer_id') as string;
		const title = formData.get('title') as string;
		const description = formData.get('description') as string;
		const job_type = formData.get('job_type') as string;
		const priority = formData.get('priority') as string;
		const assigned_to = formData.get('assigned_to') as string;
		const scheduled_start = formData.get('scheduled_start') as string;
		const estimated_duration = formData.get('estimated_duration') as string;
		const source = formData.get('source') as string;
		const po_number = formData.get('po_number') as string;

		if (!customer_id) return fail(400, { error: 'Customer is required' });
		if (!title?.trim()) return fail(400, { error: 'Job title is required' });

		try {
			const { data } = await serverPost<{ id: string }>('/jobs', {
				customer_id,
				title: title.trim(),
				description: description?.trim() || null,
				job_type: job_type || null,
				priority: priority || 'normal',
				assigned_to: assigned_to || null,
				scheduled_date: scheduled_start ? scheduled_start.split('T')[0] : null,
				scheduled_start_time: scheduled_start ? scheduled_start.split('T')[1] : null,
				estimated_duration_minutes: estimated_duration ? parseInt(estimated_duration) : null,
				source: source?.trim() || null,
				po_number: po_number?.trim() || null,
			}, { token });

			redirect(303, `/dashboard/jobs/${(data as { id: string }).id}`);
		} catch (e) {
			if (e instanceof Response || (e && typeof e === 'object' && 'status' in e)) throw e;
			return fail(500, { error: e instanceof Error ? e.message : 'Failed to create job' });
		}
	}
};
