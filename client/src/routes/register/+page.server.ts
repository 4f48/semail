import type { PageServerLoad, Actions } from './$types.js';
import { fail } from '@sveltejs/kit';
import { superValidate } from 'sveltekit-superforms';
import { register } from '@/forms';
import { zod } from 'sveltekit-superforms/adapters';

import * as opaque from "@serenity-kit/opaque";

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

		const { clientRegistrationState, registrationRequest } = opaque.client.startRegistration({ password: form.data.password });

		console.debug(registrationRequest);

		// store backend URL later in .env
		const start = await fetch('http://localhost:25052/auth/register/start', {
			method: 'POST',
			headers: { 'Content-Type': 'application/json' },
			body: JSON.stringify({
				username: form.data.username,
				request: Buffer.from(registrationRequest, "base64").toString("base64"),
			})
		});

		if (!start.ok) {
			console.error(await start.json());
			return fail(start.status, {
				form
			});
		}

		let body = await start.json();

		const { registrationRecord } = opaque.client.finishRegistration({
			clientRegistrationState,
			registrationResponse: body.response,
			password: form.data.password
		})

		const finish = await fetch('http://localhost:25052/auth/register/finish', {
			method: 'POST',
			headers: { 'Content-Type': 'application/json' },
			body: JSON.stringify({
				flow_id: body.flow_id,
				result: Buffer.from(registrationRecord).toString('base64')
			})
		});

		if (!finish.ok) {
			console.debug(await finish.json())
			return fail(finish.status, {
				form
			});
		}

		console.debug(await finish.json());

		return {
			form
		};
	}
};
