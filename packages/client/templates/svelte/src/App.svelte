<script>
	export let name;

	export let appWebsocket;
	export let cell_id;

	let entryHash;

	$: entryHash;

	 appWebsocket.callZome({
		cap: null,
		cell_id: cell_id,
		zome_name: 'HC_SCAFFOLDING{zomeName}',
		fn_name: 'HC_SCAFFOLDING{fnName}',
    payload: HC_SCAFFOLDING{entrySample},
		provenance: cell_id[1],
	}).then(hash => entryHash = hash);

</script>

<main>
	<h1>Hello {name}!</h1>
	<p>Visit the <a href="https://svelte.dev/tutorial">Svelte tutorial</a> to learn how to build Svelte apps.</p>

	{#if entryHash}
		<span>Created new Holochain entry! HC_SCAFFOLDING{entryDefName} with hash {entryHash}</span>
	{:else}
		<span>Creating...</span>
	{/if}
</main>

<style>
	main {
		text-align: center;
		padding: 1em;
		max-width: 240px;
		margin: 0 auto;
	}

	h1 {
		color: #ff3e00;
		text-transform: uppercase;
		font-size: 4em;
		font-weight: 100;
	}

	@media (min-width: 640px) {
		main {
			max-width: none;
		}
	}
</style>