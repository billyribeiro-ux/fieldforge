import type { ServerLoad, Actions } from '@sveltejs/kit';
import { fail, redirect, isRedirect } from '@sveltejs/kit';
import { serverFetch, serverPost } from '$lib/api/server';

export const load: ServerLoad = async ({ locals, url }) => {
	const token = locals.token;
	const customerId = url.searchParams.get('customer_id') ?? '';
	const jobId = url.searchParams.get('job_id') ?? '';

	try {
		const [customersRes] = await Promise.allSettled([
			serverFetch<unknown[]>('/customers', { token, params: { limit: '100' } })
		]);

		return {
			customers: customersRes.status === 'fulfilled' ? customersRes.value.data : [],
			prefill: { customerId, jobId }
		};
	} catch {
		return {
			customers: [],
			prefill: { customerId, jobId }
		};
	}
};

export const actions: Actions = {
	create: async ({ request, locals }) => {
		const token = locals.token;
		if (!token) return fail(401, { error: 'Not authenticated' });

		const formData = await request.formData();
		const customer_id = formData.get('customer_id') as string;
		const job_id = formData.get('job_id') as string;
		const title = formData.get('title') as string;
		const line_items = formData.get('line_items') as string;

		if (!customer_id) return fail(400, { error: 'Customer is required' });
		if (!title?.trim()) return fail(400, { error: 'Title is required' });

		let items = [];
		try {
			items = line_items ? JSON.parse(line_items) : [];
		} catch {
			return fail(400, { error: 'Invalid line items' });
		}

		try {
			const { data } = await serverPost<{ id: string }>('/estimates', {
				customer_id,
				job_id: job_id || null,
				title: title.trim(),
				line_items: items
			}, { token });

			redirect(303, `/dashboard/estimates/${(data as { id: string }).id}`);
		} catch (e) {
			if (isRedirect(e)) throw e;
			return fail(500, { error: e instanceof Error ? e.message : 'Failed to create estimate' });
		}
	}
};
