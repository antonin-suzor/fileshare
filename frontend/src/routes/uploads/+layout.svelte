<script lang="ts">
    import { requireVerifiedUser } from '$lib/api/auth.svelte';
    import { onMount } from 'svelte';

    let { children } = $props();

    onMount(requireVerifiedUser);
</script>

{@render children()}
