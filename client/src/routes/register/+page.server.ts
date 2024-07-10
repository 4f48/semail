import type { PageServerLoad, Actions } from './$types.js';
import { fail } from '@sveltejs/kit';
import { superValidate } from 'sveltekit-superforms';
import { register } from '@/forms';
import { zod } from 'sveltekit-superforms/adapters';

import { Registration } from '@47ng/opaque-client';
import { request } from 'http';

export const load: PageServerLoad = async () => {
	return {
		form: await superValidate(zod(register))
	};
};

export const actions: Actions = {
	default: async (event) => {
		const form = await superValidate(event, zod(register));
		if (!form.valid) {
			return fail(400, {
				form
			});
		}

		const registration = new Registration();
		const registrationRequest = registration.start(form.data.password);

		// store backend URL later in .env
		const response = await fetch('http://localhost:25052/auth/register/request', {
			method: 'POST',
			headers: { 'Content-Type': 'application/json' },
			body: JSON.stringify({
				uesrname: form.data.username,
				request: registrationRequest
			})
		});

		

		// console.debug(response);

		return {
			form
		};
	}
};
