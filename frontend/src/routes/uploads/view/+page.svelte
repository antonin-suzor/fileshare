<script lang="ts">
    import { goto } from '$app/navigation';
    import { page } from '$app/state';
    import { getUpload } from '$lib/api/uploads.svelte';
    import UploadCard from '$lib/components/UploadCard.svelte';
    import type { Upload } from '$lib/types';
    import { onMount } from 'svelte';

    let id: string | null = $state(null);
    let upload: Upload | null = $state(null);
    let fetchState: string = $state('loading');

    onMount(async () => {
        id = page.url.searchParams.get('id');
        if (id === null) {
            alert('Sorry, it looks like the URL is invalid.');
            return;
        }
        upload = await getUpload(id);
        fetchState = 'done';
    });
</script>

<svelte:head>
    <title>View Upload | FileShare</title>
</svelte:head>

<a href="/uploads" class="link">Back to upload list</a>

<div class="flex justify-center-safe">
    {#if fetchState === 'loading'}
        <div>Loading...</div>
    {:else if fetchState === 'done' && upload !== null}
        <UploadCard {upload} onDelete={() => goto('/uploads')} />
    {:else}
        <div>We will soon get the information you need.</div>
    {/if}
</div>
