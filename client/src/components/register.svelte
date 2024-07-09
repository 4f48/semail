<script lang="ts">
	import * as Form from '@/components/ui/form';
	import { Input } from '@/components/ui/input';
	import { Button } from '@/components/ui/button';
	import { register, type Register } from '@/forms';
	import { type SuperValidated, type Infer, superForm } from 'sveltekit-superforms';
	import { zodClient } from 'sveltekit-superforms/adapters';

	export let data: SuperValidated<Infer<Register>>;
	const form = superForm(data, {
		validators: zodClient(register)
	});

	const { form: formData, enhance } = form;
</script>

<form method="POST" use:enhance class="flex w-[24rem] flex-col gap-2">
	<Form.Field {form} name="username">
		<Form.Control let:attrs>
			<Form.Label>Username</Form.Label>
			<Input {...attrs} bind:value={$formData.username} type="text" />
		</Form.Control>
	</Form.Field>
	<Form.Field {form} name="password">
		<Form.Control let:attrs>
			<Form.Label>Password</Form.Label>
			<Input {...attrs} bind:value={$formData.password} type="password" />
		</Form.Control>
	</Form.Field>
	<Form.Button>Register</Form.Button>
	<p class="flex items-center gap-1 self-center text-sm text-muted-foreground">
		Already have an account? <a href="/login"
			><Button variant="link" class="m-0 p-0">Log in</Button></a
		>
	</p>
</form>
