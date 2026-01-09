<script lang="ts">
    import { goto } from '$app/navigation';
    import { axiosInstance } from '$lib/api/axios';

    let files: FileList | null = $state(null);

    async function uploadFiles() {
        const file: File = files?.item(0)!;
        console.log(file);
        let res = await axiosInstance.post('/api/uploads/start', {
            file_name: file.name,
            content_type: file.type,
            expires_at: new Date(Date.now() + 1000 * 60 * 60 * 24),
        });
        console.log(res.status, res.data);
        const response = await fetch(res.data.url, {
            method: 'PUT',
            body: file,
            headers: {
                'Content-Type': file.type,
            },
        });
        goto('/uploads');
    }
</script>

<svelte:head>
    <title>New Upload | FileShare</title>
</svelte:head>

<div class="flex justify-center-safe">
    <input type="file" class="file-input file-input-ghost" bind:files />
    <button onclick={uploadFiles} class="btn" disabled={files === null}>Upload file</button>
</div>
