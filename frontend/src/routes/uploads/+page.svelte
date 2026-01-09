<script lang="ts">
    import { pushState } from '$app/navigation';
    import { page } from '$app/state';
    import { axiosInstance } from '$lib/api/axios';
    import UploadCard from '$lib/components/UploadCard.svelte';
    import type { Upload } from '$lib/types';
    import { onMount } from 'svelte';

    let uploads: Upload[] = $state([]);
    let searchQuery: string = $state('');

    function removeUpload(upload: Upload) {
        uploads = uploads.filter((u) => u.id !== upload.id);
    }

    function handleSearch() {
        const params = new URLSearchParams();
        if (searchQuery.trim()) {
            params.set('search', searchQuery);
        }
        pushState(`?${params.toString()}`, page.state);
    }

    function filteredUploads(): Upload[] {
        if (!searchQuery.trim()) {
            return uploads;
        }
        const query = searchQuery.toLowerCase();
        return uploads.filter((upload) => upload.file_name.toLowerCase().includes(query));
    }

    onMount(async () => {
        uploads = (await axiosInstance.get('/api/uploads/mine')).data;
        searchQuery = page.url.searchParams.get('search') ?? '';
    });
</script>

<svelte:head>
    <title>Uploads | FileShare</title>
</svelte:head>

<div class="flex justify-center-safe">
    <span class="join">
        <input bind:value={searchQuery} class="input join-item rounded-l-full" placeholder="Search bar" />
        <button onclick={handleSearch} class="btn join-item rounded-r-full">Search</button>
    </span>
    <a href="/uploads/new" class="btn rounded-full">New upload</a>
</div>

<div class="flex flex-row flex-wrap">
    {#each filteredUploads() as upload (upload.id)}
        <UploadCard {upload} onDelete={removeUpload} />
    {/each}
</div>
