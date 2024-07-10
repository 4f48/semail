import type { PageServerLoad, Actions } from './$types.js';
import { fail } from '@sveltejs/kit';
import { superValidate } from 'sveltekit-superforms';
import { register } from '@/forms';
import { zod } from 'sveltekit-superforms/adapters';

import { Registration } from '@47ng/opaque-server';
import * as bincode from "bincode-ts";

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
		const start = await fetch('http://localhost:25052/auth/register/start', {
			method: 'POST',
			headers: { 'Content-Type': 'application/json' },
			body: JSON.stringify({
				username: form.data.username,
				request: Buffer.from(registrationRequest).toString('base64')
			})
		});

		if (!start.ok) {
			return fail(start.status, {
				form
			});
		}

		let body = await start.json();

		let decoder = new bincode.Decoder();

		console.debug({
			base64: body.response,
			converted: ""
		})

		const registrationRecord = registration.finish(
			form.data.password,
			Uint8Array.from(atob(body.response), (c) => c.charCodeAt(0))
		);

		const finish = await fetch('http://localhost:25052/auth/register/finish', {
			method: 'POST',
			headers: { 'Content-Type': 'application/json' },
			body: JSON.stringify({
				flow_id: body.flow_id,
				result: Buffer.from(registrationRecord).toString('base64')
			})
		});

		if (!finish.ok) {
			return fail(finish.status, {
				form
			});
		}

		return {
			form
		};
	}
};
