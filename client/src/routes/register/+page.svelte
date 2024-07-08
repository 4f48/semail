<script lang="ts">
	import type { PageData } from './$types.js';
	import Register from '#/register.svelte';
	export let data: PageData;
</script>

<div class="flex h-[100%] w-[100%] flex-col items-center justify-center gap-10">
	<h1 class="text-3xl font-semibold">Create your account</h1>
	<Register data={data.form} />
</div>
