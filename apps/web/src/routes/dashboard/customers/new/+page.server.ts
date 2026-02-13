import type { ServerLoad, Actions } from '@sveltejs/kit';
import { fail, redirect } from '@sveltejs/kit';
import { serverPost } from '$lib/api/server';

export const load: ServerLoad = async () => {
	return {};
};

export const actions: Actions = {
	create: async ({ request, locals }) => {
		const token = locals.token;
		if (!token) return fail(401, { error: 'Not authenticated' });

		const formData = await request.formData();
		const first_name = formData.get('first_name') as string;
		const last_name = formData.get('last_name') as string;
		const email = formData.get('email') as string;
		const phone = formData.get('phone') as string;
		const company_name = formData.get('company_name') as string;
		const customer_type = formData.get('customer_type') as string;
		const preferred_contact_method = formData.get('contact_method') as string;
		const credit_terms = formData.get('credit_terms') as string;
		const notes = formData.get('notes') as string;
		const source = formData.get('source') as string;
		const address_line1 = formData.get('address_line1') as string;
		const address_line2 = formData.get('address_line2') as string;
		const city = formData.get('city') as string;
		const state = formData.get('state') as string;
		const zip_code = formData.get('zip_code') as string;

		if (!first_name?.trim()) return fail(400, { error: 'First name is required' });
		if (!last_name?.trim()) return fail(400, { error: 'Last name is required' });

		try {
			const { data } = await serverPost<{ id: string }>('/customers', {
				first_name: first_name.trim(),
				last_name: last_name.trim(),
				email: email?.trim() || null,
				phone: phone?.trim() || null,
				company_name: company_name?.trim() || null,
				customer_type: customer_type || 'residential',
				preferred_contact_method: preferred_contact_method || null,
				credit_terms: credit_terms || null,
				notes_pinned: notes?.trim() || null,
				source: source || null,
				address: address_line1?.trim() ? {
					address_line1: address_line1.trim(),
					address_line2: address_line2?.trim() || null,
					city: city?.trim() || null,
					state: state?.trim() || null,
					zip_code: zip_code?.trim() || null,
				} : null,
			}, { token });

			redirect(303, `/dashboard/customers/${(data as { id: string }).id}`);
		} catch (e) {
			if (e instanceof Response || (e && typeof e === 'object' && 'status' in e)) throw e;
			return fail(500, { error: e instanceof Error ? e.message : 'Failed to create customer' });
		}
	}
};
