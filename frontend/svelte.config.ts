import adapter from '@sveltejs/adapter-static';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';
import { Config } from '@sveltejs/kit';

const config: Config = {
    preprocess: vitePreprocess(),
    kit: {
        adapter: adapter({
            pages: 'build',
            assets: 'build',
            fallback: undefined,
            precompress: false,
            strict: false,
        }),
        prerender: {
            crawl: true,
            handleHttpError: 'fail',
            handleMissingId: 'warn',
            handleEntryGeneratorMismatch: 'fail',
            handleUnseenRoutes: 'warn',
        },
    },
};

export default config;
