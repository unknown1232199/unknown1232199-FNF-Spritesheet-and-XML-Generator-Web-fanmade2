<script lang="ts">
    import AppTab from "./components/AppTab.svelte";
    import IcongridGenerator from "./tabs/IcongridGenerator.svelte";
    import SpritesheetGenerator from "./tabs/SpritesheetGenerator.svelte";
    import type { Wasm_T } from './global';

    export let wasm: Wasm_T;
    let characterName = '';
    let curChoice = "spsh";
    let darkMode = true;
</script>

<svelte:head>
    <title>FNF Spritesheet and XML Generator Web — Fan Version (Fantake)</title>
    <meta name="description" content="Fantake, an independent fan version of FNF Spritesheet and XML Generator Web." />
</svelte:head>

<main>
    <header class="app-header">
        <div class="brand">
            <span class="eyebrow">FANTAKE / SPRITE TOOLS</span>
            <h1 id="title">FNF Spritesheet &amp; XML Generator</h1>
            <p>Prepare, organize and export your Friday Night Funkin' sprites.</p>
        </div>
        <label class="theme-toggle" for="dark-mode">
            <input type="checkbox" name="dark-mode" id="dark-mode" bind:checked={darkMode} on:change={() => { document.body.classList.toggle('light-mode', !darkMode); }}>
            <span>{darkMode ? 'Dark mode' : 'Light mode'}</span>
        </label>
    </header>

    <section class="character-card" aria-label="Character settings">
        <label for="char-name">Character name</label>
        <input type="text" name="char-name" id="char-name" bind:value={characterName} placeholder="Enter character name…" autocomplete="off" />
        <small>This name is used for the exported files.</small>
    </section>

    <nav id="app-tabs" aria-label="Generator type">
        <AppTab id="spsh" bind:curChoice text="Spritesheet Generator" />
        <AppTab id="icongrid" bind:curChoice text="Icon Grid Generator" />
    </nav>

    <section id="tab-container">
        {#if curChoice === "spsh"}
            <SpritesheetGenerator charname={characterName} wasm={wasm} />
        {:else if curChoice === "icongrid"}
            <IcongridGenerator bind:charname={characterName} wasm={wasm} />
        {/if}
    </section>

    <footer>FANTAKE · Fan version · Original project credited in the README</footer>
</main>

<style>
    main {
        width: min(1120px, calc(100% - 28px));
        margin: 0 auto;
        padding: 22px 0 18px;
    }
    .app-header {
        display: flex;
        align-items: flex-start;
        justify-content: space-between;
        gap: 18px;
        margin-bottom: 18px;
    }
    .brand { min-width: 0; }
    .eyebrow {
        display: block;
        margin-bottom: 7px;
        color: var(--text-color);
        opacity: .65;
        font-size: .72rem;
        font-weight: 750;
        letter-spacing: .14em;
    }
    #title {
        margin: 0;
        font-size: clamp(1.35rem, 3.4vw, 2rem);
        line-height: 1.2;
        text-align: left;
    }
    .brand p {
        margin: 7px 0 0;
        opacity: .72;
        font-size: .92rem;
    }
    .theme-toggle {
        display: inline-flex;
        align-items: center;
        gap: 8px;
        flex: 0 0 auto;
        padding: 9px 12px;
        border: 1px solid var(--border-color);
        border-radius: 999px;
        cursor: pointer;
        white-space: nowrap;
    }
    .character-card {
        display: grid;
        grid-template-columns: 150px minmax(0, 1fr);
        align-items: center;
        gap: 8px 14px;
        padding: 14px 16px;
        margin-bottom: 16px;
        border: 1px solid var(--border-color);
        border-radius: 12px;
        background: var(--background);
    }
    .character-card label { font-weight: 700; }
    .character-card input {
        box-sizing: border-box;
        width: 100%;
        min-width: 0;
        padding: 10px 12px;
        border: 1px solid var(--border-color);
        border-radius: 8px;
        background: var(--background);
        color: var(--text-color);
        font: inherit;
    }
    .character-card small {
        grid-column: 2;
        opacity: .65;
    }
    #app-tabs {
        display: grid;
        grid-template-columns: repeat(2, minmax(0, 1fr));
        gap: 6px;
        margin-bottom: 0;
    }
    #tab-container {
        min-width: 0;
        border: 1px solid var(--border-color);
        border-radius: 0 0 12px 12px;
        overflow: visible;
    }
    footer {
        padding: 16px 4px 0;
        text-align: center;
        font-size: .75rem;
        opacity: .55;
    }
    @media (max-width: 600px) {
        main { width: min(100% - 18px, 1120px); padding-top: 14px; }
        .app-header { flex-direction: column; }
        .theme-toggle { align-self: flex-start; }
        .character-card { grid-template-columns: 1fr; gap: 7px; }
        .character-card small { grid-column: 1; }
        #app-tabs { gap: 4px; }
    }
</style>